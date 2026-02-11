use anyhow::Ok;
use serde::Deserialize;

use crate::{key::ValueKey, module::Module};

const MC_VERSION: ValueKey = ValueKey("MC_VERSION");
const YARN_VERSION: ValueKey = ValueKey("YARN_VERSION");
const LOADER_VERSION: ValueKey = ValueKey("LOADER_VERSION");
const LOOM_VERSION: ValueKey = ValueKey("LOOM_VERSION");
const API_VERSION: ValueKey = ValueKey("API_VERSION");
#[derive(Debug, Clone, Deserialize, Default)]
pub struct VersionModule {
    minecraft: String,
    yarn: String,
    loom: String,
    loader: String,
    api: String,
}
impl Module for VersionModule {
    fn files(&self) -> &'static [crate::key::FileKey] {
        &[]
    }

    fn export(&self, exports: &mut super::ExportedValues) -> anyhow::Result<()> {
        exports.set(MC_VERSION, &self.minecraft);
        exports.set(YARN_VERSION, &self.yarn);
        exports.set(LOADER_VERSION, &self.loader);
        exports.set(LOOM_VERSION, &self.loom);
        exports.set(API_VERSION, &self.api);
        Ok(())
    }

    fn show(&mut self, ui: &mut egui::Ui) {
        let response = ui.link("Check dependency versions on https://fabricmc.net/develop/");
        if response.clicked() {
            open::that("https://fabricmc.net/develop/").unwrap();
        } else if response.hovered() {
            ui.label("Or in an eg. Chat channel for your modding project");
        }
        ui.separator();
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

    fn create_default() -> Self
    where
        Self: Sized,
    {
        VersionModule::default()
    }
}
