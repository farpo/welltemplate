use eframe::App;
use egui::{ThemePreference, Ui};
use std::{
    cell::RefCell,
    collections::HashMap,
    fs::{self},
    path::{Path, PathBuf},
    rc::Rc,
    time::Instant,
};

use crate::{
    files::{root_files, src_files},
    modules::{
        DATAGEN_INIT_KEY, ENTRYPOINT_IMPORT_KEY, ENTRYPOINT_INIT_KEY,
        block::Block,
        datagen::{lang, loot_table, model, recipe, tag},
        entity::Entity,
        item::Item,
        metadata::Metadata,
        network::Network,
        version::VersionData,
    },
};

type ModuleData = Rc<RefCell<dyn Module>>;
type Transformations = HashMap<&'static str, String>;

mod files;
mod modules;
fn main() {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            //.with_icon(load_icon())
            .with_inner_size([960.0, 720.0]),
        ..Default::default()
    };
    eframe::run_native(
        "Welltemplate",
        options,
        Box::new(|_cc| Ok(Box::<Welltemplate>::default())),
    )
    .expect("Did not gui");
}
/*fn load_icon() -> IconData {
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
}*/
struct Welltemplate {
    generation_path: Option<PathBuf>,
    modid: String,
    modname: String,
    modgroup: String,
    currently_shown_module: Option<(String, Rc<RefCell<dyn Module>>)>,
    module_map: HashMap<String, ModuleData>,
    log: Vec<String>,
}
impl App for Welltemplate {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        ctx.set_theme(ThemePreference::Dark);
        if let Some((_, b)) = &mut self.currently_shown_module {
            egui::SidePanel::right("module").show(ctx, |ui| b.borrow_mut().show_panel(ui));
        }
        egui::TopBottomPanel::bottom("log").show(ctx, |ui| {
            for string in self.log.iter() {
                ui.label(string);
            }
        });
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.horizontal(|ui| {
                if ui.button("Select generation location").clicked() {
                    if let Some(path) = rfd::FileDialog::new().pick_folder() {
                        self.generation_path = Some(path);
                    }
                }
                if ui.button("Generate").clicked() {
                    self.generate();
                }
                if ui.button("Save Module").clicked() {
                    self.save_module();
                };
            });
            if let Some(path) = &self.generation_path {
                ui.label(format!("Generation path: {}", path.display()));
            }
            ui.separator();
            ui.horizontal(|ui| {
                if ui.button("Versions").clicked() {
                    self.create_module("version", VersionData::default());
                }
                if ui.button("Metadata").clicked() {
                    self.create_module("metadata", Metadata::default());
                }
            });
            ui.separator();
            ui.label("Mod Id");
            ui.text_edit_singleline(&mut self.modid);
            ui.separator();
            ui.label("Mod Name");
            ui.text_edit_singleline(&mut self.modname);
            ui.separator();
            ui.label("Mod Group");
            ui.text_edit_singleline(&mut self.modgroup);
            ui.separator();
            ui.label("Content modules");
            if ui.button("Item module").clicked() {
                self.create_module("item", Item("Mod".to_owned()));
            }
            if ui.button("Block module").clicked() {
                self.create_module("block", Block("Mod".to_owned()));
            }
            if ui.button("Entity module").clicked() {
                self.create_module("entity", Entity("Mod".to_owned()));
            }
            if ui.button("Network module").clicked() {
                self.create_module("network", Network("Mod".to_owned()));
            }
            ui.separator();
            ui.label("Datagen modules");
            ui.horizontal(|ui| {
                if ui.button("Lang module").clicked() {
                    self.add_module("lang", lang::Lang);
                }
                if ui.button("Model module").clicked() {
                    self.create_module("model", model::Model(false));
                }
                if ui.button("Tag module").clicked() {
                    self.create_module("tag", tag::Tag);
                }
                if ui.button("Loot Table module").clicked() {
                    self.create_module("loot_table", loot_table::LootTable);
                }
                if ui.button("Recipe module").clicked() {
                    self.create_module("recipe", recipe::Recipe);
                }
            })
        });
    }
}
impl Default for Welltemplate {
    fn default() -> Self {
        let mut template = Welltemplate {
            generation_path: None,
            modid: "".to_owned(),
            modname: "".to_owned(),
            modgroup: "".to_owned(),
            currently_shown_module: None,
            module_map: HashMap::new(),
            log: vec![],
        };
        template.add_module("version", VersionData::default());
        template.add_module("metadata", Metadata::default());
        template
    }
}
impl FileTransformer for Welltemplate {
    fn transform(&self, transformations: &mut Transformations) {
        transformations.insert("``MOD_NAME``", self.modname.clone());
        transformations.insert("``MOD_ID``", self.modid.clone());
        transformations.insert("``MOD_GROUP``", self.modgroup.clone());
        let mut iter = self.modgroup.split(".").collect::<Vec<&str>>();
        iter.remove(iter.len() - 1);
        transformations.insert("``MOD_GROUP_SUFFIXLESS``", iter.join("."));
        transformations.insert(ENTRYPOINT_INIT_KEY, String::new());
        transformations.insert(ENTRYPOINT_IMPORT_KEY, String::new());
        transformations.insert(DATAGEN_INIT_KEY, String::new());
    }
}
fn is_valid_id(id: &str) -> bool {
    if id.is_empty() {
        return false;
    }
    for char in id.chars() {
        if !is_valid_id_char(char) {
            return false;
        }
    }
    true
}
fn is_valid_id_char(char: char) -> bool {
    char == '_' || char == '-' || char.is_ascii_lowercase() || char.is_ascii_digit() || char == '.'
}
fn is_valid_group(group: &str) -> bool {
    if group.is_empty() {
        return false;
    }
    for char in group.chars() {
        if !is_valid_group_char(char) {
            return false;
        }
    }
    group.split('.').collect::<Vec<&str>>().len() > 1
}
fn is_valid_group_char(char: char) -> bool {
    char.is_ascii_lowercase() || char == '.'
}
impl Welltemplate {
    fn validate(&mut self) -> bool {
        self.log.clear();
        let mut bool = true;
        if self.generation_path.is_none() {
            self.log("Invalid: No generation path set!".to_owned());
            bool = false;
        }
        if self.modname.is_empty() || self.modname.contains(' ') {
            self.log("Invalid: Mod name must not contain spaces and must not be empty".to_owned());
            bool = false;
        }
        if !is_valid_id(&self.modid) {
            self.log("Invalid: Mod id must be a valid Minecraft Namespace: Only contain lowercase ASCII, numbers, dashes, underscores and dots".to_owned());
            bool = false;
        }
        if !is_valid_group(&self.modgroup) {
            self.log("Invalid: Mod group must be a valid Java Package: Only contain lowercase ASCII and dots and be longer than one segment".to_owned());
            bool = false;
        }
        bool
    }
    fn generate(&mut self) {
        if self.validate() {
            let start = Instant::now();
            self.log(format!(
                "Started generating to folder {}",
                self.generation_path.as_ref().unwrap().display()
            ));
            self.save_module();
            let mut transformations: Transformations = HashMap::new();
            self.transform(&mut transformations);
            let entrypoint_name = self.modname.clone().replace(" ", "");
            transformations.insert("``ENTRYPOINT_NAME``", entrypoint_name.clone());
            for (_, module) in self.module_map.iter() {
                module.clone().borrow().transform(&mut transformations);
            }
            let path = self.generation_path.as_ref().unwrap();
            src_files::write_resources(path, &transformations);
            src_files::write_entrypoints(path, &transformations, entrypoint_name.as_str());
            root_files::write(path, &transformations);
            for (_, module) in self.module_map.iter() {
                module
                    .clone()
                    .borrow()
                    .write_templates(path, &transformations);
            }
            let duration = Instant::now().duration_since(start);
            self.log(format!("Finished generating in {:?}", duration));
        }
    }
    fn save_module(&mut self) {
        if let Some((name, b)) = &self.currently_shown_module {
            self.module_map.insert(name.clone(), b.clone());
            self.log(format!("Saved {name} Module"));
        }
        self.currently_shown_module = None;
    }
    fn create_module(&mut self, name: &str, module: impl Module + 'static) {
        self.save_module();
        self.currently_shown_module = self
            .module_map
            .get(name)
            .cloned()
            .or(Some(Rc::new(RefCell::new(module))))
            .map(|o| (name.to_owned(), o));
        self.log(format!("Displayed {name} Module"));
    }
    fn add_module(&mut self, name: &str, module: impl Module + 'static) {
        if !self.module_map.contains_key(name) {
            self.module_map
                .insert(name.to_owned(), Rc::new(RefCell::new(module)));
        }
        self.log(format!("Added {name} Module"));
    }
    fn log(&mut self, string: String) {
        if self.log.len() > 5 {
            self.log.remove(0);
        }
        self.log.push(string);
    }
}

pub trait FileTransformer {
    fn transform(&self, transformations: &mut Transformations);
}
pub trait Module: FileTransformer {
    fn write_templates(&self, path: &Path, transformations: &Transformations);
    fn show_panel(&mut self, ui: &mut Ui);
}

const fn template(name: &'static str, file: &'static str) -> Template {
    template_bin(name, file.as_bytes())
}
const fn template_bin(name: &'static str, file: &'static [u8]) -> Template {
    Template { name, file }
}
pub struct Template {
    name: &'static str,
    file: &'static [u8],
}
impl Template {
    pub fn write(&self, path: &Path, transformations: &Transformations) {
        self.write_internal(path, transformations, None);
    }
    fn write_internal(&self, path: &Path, transformations: &Transformations, name: Option<&str>) {
        let result = String::from_utf8(self.file.to_vec());
        let bytes = match result {
            Ok(mut string) => {
                for (key, value) in transformations.iter() {
                    string = string.replace(key, value)
                }
                string.as_bytes().to_vec()
            }
            Err(_) => self.file.to_vec(),
        };
        let this_path = match name {
            Some(name) => path.join(self.name).join(name),
            None => path.join(self.name),
        };
        fs::create_dir_all(this_path.parent().unwrap()).unwrap();
        fs::write(path.join(this_path), bytes).unwrap();
    }
    pub fn write_named(&self, path: &Path, transformations: &Transformations, name: &str) {
        self.write_internal(path, transformations, Some(name));
    }
}
