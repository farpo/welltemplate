use std::{fmt::Display, path::Path};

use egui::Ui;

use crate::{FileTransformer, Module, Transformations};
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum McVersion {
    TWENTYONEONE,
    TWENTYONEEIGHT,
}

impl Display for McVersion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                McVersion::TWENTYONEONE => "1.21.1",
                McVersion::TWENTYONEEIGHT => "1.21.8",
            }
        )
    }
}
impl From<&String> for McVersion {
    fn from(value: &String) -> Self {
        match value.as_str() {
            "1.21.1" => McVersion::TWENTYONEONE,
            _ => McVersion::TWENTYONEEIGHT,
        }
    }
}
pub struct VersionData {
    minecraft: McVersion,
    yarn: String,
    loom: String,
    loader: String,
    api: String,
}
impl Default for VersionData {
    fn default() -> Self {
        VersionData::recommended(McVersion::TWENTYONEEIGHT)
    }
}
impl VersionData {
    fn recommended(mc: McVersion) -> Self {
        match mc {
            McVersion::TWENTYONEONE => VersionData {
                minecraft: McVersion::TWENTYONEONE,
                yarn: "1.21.1+build.3".to_owned(),
                loom: "1.11-SNAPSHOT".to_owned(),
                loader: "0.16.4".to_owned(),
                api: "0.116.5+1.21.1".to_owned(),
            },
            McVersion::TWENTYONEEIGHT => VersionData {
                minecraft: McVersion::TWENTYONEEIGHT,
                yarn: "1.21.8+build.1".to_owned(),
                loom: "1.11-SNAPSHOT".to_owned(),
                loader: "0.17.2".to_owned(),
                api: "0.132.0+1.21.8".to_owned(),
            },
        }
    }
    fn set_recommended(&mut self) {
        let other = VersionData::recommended(self.minecraft);
        self.api = other.api;
        self.loader = other.loader;
        self.loom = other.loom;
        self.yarn = other.yarn;
    }
}
impl Module for VersionData {
    fn write_templates(&self, _path: &Path, _transformations: &Transformations) {}

    fn show_panel(&mut self, ui: &mut Ui) {
        egui::ComboBox::from_label("Minecraft version")
            .selected_text(format!("{}", self.minecraft))
            .show_ui(ui, |ui| {
                ui.selectable_value(&mut self.minecraft, McVersion::TWENTYONEEIGHT, "1.21.8");
                ui.selectable_value(&mut self.minecraft, McVersion::TWENTYONEONE, "1.21.1");
            });
        ui.separator();
        let response = ui.link("Check dependency versions on https://fabricmc.net/develop/");
        if response.clicked() {
            open::that("https://fabricmc.net/develop/").unwrap();
        } else if response.hovered() {
            ui.label("Or in an eg. Chat channel for your modding project");
        }
        if ui.button("Set recommended").clicked() {
            self.set_recommended();
        }
        ui.separator();
        let id = ui.label("Loom version").id;
        ui.text_edit_singleline(&mut self.loom).labelled_by(id);
        ui.separator();
        let id = ui.label("Yarn version").id;
        ui.text_edit_singleline(&mut self.yarn).labelled_by(id);
        ui.separator();
        let id = ui.label("Loader version").id;
        ui.text_edit_singleline(&mut self.loader).labelled_by(id);
        ui.separator();
        let id = ui.label("Fabric API version").id;
        ui.text_edit_singleline(&mut self.api).labelled_by(id);
    }
}
impl FileTransformer for VersionData {
    fn transform(&self, transformations: &mut Transformations) {
        transformations.insert("``MC_VERSION``", self.minecraft.to_string());
        transformations.insert("``YARN_VERSION``", self.yarn.clone());
        transformations.insert("``LOOM_VERSION``", self.loom.clone());
        transformations.insert("``LOADER_VERSION``", self.loader.clone());
        transformations.insert("``API_VERSION``", self.api.clone());
    }
}
