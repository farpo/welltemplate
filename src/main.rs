mod key;
#[cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
mod module;
mod template;

use anyhow::{Error, Result, bail};
use eframe::App;
use egui::{IconData, ThemePreference};
use itertools::Itertools;
use std::{collections::HashMap, fs, path::PathBuf};

use crate::{
    key::FileKey,
    module::{
        ExportedValues, Module, OptionalModule, OptionalModuleKey, metadata::MetadataModule, name::NameModule, version::VersionModule
    },
    template::TemplateDefinition,
};

fn main() {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_icon(load_icon())
            .with_inner_size([960.0, 720.0]),
        ..Default::default()
    };
    eframe::run_native(
        "Welltemplate",
        options,
        Box::new(|_cc| Ok(Box::new(Welltemplate::create(template::load())))),
    )
    .expect("Did not gui");
}
fn load_icon() -> IconData {
    let (icon_rgba, icon_width, icon_height) = {
        let icon = include_bytes!("../icon.png");
        let image = image::load_from_memory(icon)
            .expect("Failed to open icon path")
            .into_rgba8();
        let (width, height) = image.dimensions();
        let rgba = image.into_raw();
        (rgba, width, height)
    };

    IconData {
        rgba: icon_rgba,
        width: icon_width,
        height: icon_height,
    }
}
#[derive(Clone, Copy, Debug)]
enum Selected {
    RootModule,
    VersionModule,
    MetadataModule,
    OptionalModule(OptionalModuleKey),
}
struct Welltemplate {
    generation_path: Option<PathBuf>,
    selected_template: Option<String>,
    templates: HashMap<String, template::TemplateDefinition>,
    selected: Option<Selected>,
    root_module: NameModule,
    version_module: VersionModule,
    metadata_module: MetadataModule,
    optional_modules: HashMap<OptionalModuleKey, Box<dyn OptionalModule>>,
    log: Vec<String>,
}
impl App for Welltemplate {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        ctx.set_theme(ThemePreference::Dark);
        /*if let Some((_, b)) = &mut self.currently_shown_module {
            egui::SidePanel::right("module").show(ctx, |ui| b.borrow_mut().show_panel(ui));
        }*/
        egui::TopBottomPanel::bottom("log").show(ctx, |ui| {
            for string in self.log.iter() {
                ui.label(string);
            }
        });
        egui::TopBottomPanel::top("controls").show(ctx, |ui| {
            ui.horizontal(|ui| {
                if ui.button("Select generation location").clicked() {
                    if let Some(path) = rfd::FileDialog::new().pick_folder() {
                        self.generation_path = Some(path);
                    }
                }
                if ui.button("Generate").clicked() {
                    self.generate();
                }
                egui::ComboBox::from_label("Select Template Set")
                    .selected_text(
                        self.selected_template
                            .as_ref()
                            .unwrap_or(&"Nothing is selected".to_owned()),
                    )
                    .show_ui(ui, |ui| {
                        for (key, template) in self.templates.iter() {
                            ui.selectable_value(
                                &mut self.selected_template,
                                Some(key.to_owned()),
                                &template.name,
                            );
                        }
                    })
            });
            if let Some(path) = &self.generation_path {
                ui.label(format!("Generation path: {}", path.display()));
            }
            ui.separator();
            ui.horizontal(|ui| {
                if ui.button("Mod").clicked() {
                    self.selected = Some(Selected::RootModule);
                }
                if ui.button("Versions").clicked() {
                    self.selected = Some(Selected::VersionModule);
                }
                if ui.button("Metadata").clicked() {
                    self.selected = Some(Selected::MetadataModule);
                }
            })
        });
        if let Some(selected) = self.selected {
            egui::CentralPanel::default().show(ctx, |ui| match selected {
                Selected::RootModule => self.root_module.show(ui),
                Selected::VersionModule => {
                    self.version_module.show(ui);
                    ui.separator();
                    if let Some(ref sel) = self.selected_template {
                        if let Some(version_module) = self
                            .templates
                            .get(sel)
                            .and_then(TemplateDefinition::get_recommendations)
                        {
                            if ui.button("Set to recommended versions").clicked() {
                                self.version_module = version_module;
                            }
                        }
                    }
                }
                Selected::MetadataModule => self.metadata_module.show(ui),
                Selected::OptionalModule(optional_module_key) => todo!(),
            });
        }
    }
}
impl Welltemplate {
    fn create(definitions: HashMap<String, template::TemplateDefinition>) -> Self {
        let template = Welltemplate {
            generation_path: None,
            selected_template: definitions
                .keys()
                .collect::<Vec<&String>>()
                .pop()
                .map(String::clone),
            templates: definitions,
            root_module: NameModule::create_default(),
            version_module: VersionModule::create_default(),
            metadata_module: MetadataModule::create_default(),
            selected: Some(Selected::RootModule),
            optional_modules: HashMap::new(),
            log: vec![],
        };
        template
    }
}

impl Welltemplate {
    fn generate(&mut self) {
        if let (Some(location), Some(template)) = (self.generation_path.clone(), &self.selected_template) {
            let mut exports = ExportedValues::default();
            let mut results = vec![
                self.root_module.export(&mut exports),
                self.version_module.export(&mut exports),
                self.metadata_module.export(&mut exports),
            ];
            for (_key, value) in self.optional_modules.iter() {
                results.push(value.export(&mut exports));
            }
            let results = results
                .into_iter()
                .filter_map(|n| n.err())
                .collect::<Vec<Error>>();
            if results.is_empty() {
                self.log(match generate(location.clone(), &self.templates, template, exports, self.process_file_keys()) {
                    Ok(_) => "Successfully generated".to_owned(),
                    Err(err) => err.to_string(),
                });
            } else {
                for r in results {
                    self.log(r.to_string());
                }
            }
        } else {
            self.log("Generation path not set!".to_owned());
        }
    }
    fn process_file_keys(&self) -> Vec<FileKey> {
        vec![self.root_module.files(), self.version_module.files(), self.metadata_module.files()]
        .into_iter()
        .chain(self.optional_modules.values().map(|module| module.files()))
        .flatten()
        .map(|k| *k)
        .unique()
        .collect::<Vec<FileKey>>()
        
    }
    fn log(&mut self, string: String) {
        if self.log.len() > 8 {
            self.log.remove(0);
        }
        self.log.push(string);
    }
}
fn generate(
    location: PathBuf,
    templates: &HashMap<String, TemplateDefinition>,
    selected: &String,
    exports: ExportedValues,
    files: Vec<FileKey>,
) -> Result<()> {
    for key in files {
        let destination = location.join(key.resolve_path(&exports));
        let source = template::find(key, selected, templates);
        if let Some(source) = source {
            fs::create_dir_all(destination.parent().unwrap())?;
            //key.2 marks a binary file
            if key.2 {
                fs::copy(source, destination)?;
            } else {
                let mut stringy = fs::read_to_string(source)?;
                for (val_key, value) in exports.iter() {
                    stringy = stringy.replace(&format!("``{}``", val_key.0), &value.resolve())
                }
                fs::write(destination, stringy)?;
            }
        } else {
            bail!("Could not write file {key:?} : not found");
        }
    }
    Ok(())
}
