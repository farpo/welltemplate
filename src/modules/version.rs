use std::path::Path;

use egui::Ui;

use crate::{FileTransformer, Module, Transformations};

pub struct VersionData {
    minecraft: String,
    yarn: String,
    loom: String,
    loader: String,
    api: String,
}
impl Default for VersionData {
    fn default() -> Self {
        VersionData {
            minecraft: "1.21.8".to_owned(),
            yarn: "1.21.8+build.1".to_owned(),
            loom: "1.11-SNAPSHOT".to_owned(),
            loader: "0.17.2".to_owned(),
            api: "0.132.0+1.21.8".to_owned(),
        }
    }
}
impl Module for VersionData {
    fn write_templates(&self, _path: &Path, _transformations: &Transformations) {}

    fn show_panel(&mut self, ui: &mut Ui) {
        let id = ui.label("Minecraft version").id;
        ui.text_edit_singleline(&mut self.minecraft).labelled_by(id);
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
        transformations.insert("``MC_VERSION``", self.minecraft.clone());
        transformations.insert("``YARN_VERSION``", self.yarn.clone());
        transformations.insert("``LOOM_VERSION``", self.loom.clone());
        transformations.insert("``LOADER_VERSION``", self.loader.clone());
        transformations.insert("``API_VERSION``", self.api.clone());
    }
}
