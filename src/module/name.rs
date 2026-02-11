use anyhow::{Ok, Result, bail};
use itertools::Itertools;

use crate::{
    key::{
        FileKey, ValueKey, common, file_key, file_key_asset, file_key_binary, file_key_modided,
        file_key_modnamed_packaged,
    },
    module::Module,
};

const MOD_NAME: ValueKey = ValueKey("MOD_NAME");
const MOD_GROUP: ValueKey = ValueKey("MOD_GROUP");
const SUFFIXELESS_GROUP: ValueKey = ValueKey("MOD_GROUP_SUFFIXLESS");

const LICENSE_HOLDER: ValueKey = ValueKey("LICENSE_HOLDER");

//This contains a lot of files (all of the base ones because this is the base module)
const GRADLE_SETTINGS: FileKey = file_key("settings.gradle");
const LICENSE: FileKey = file_key("LICENSE");
const GRADLEW_BAT: FileKey = file_key("gradlew.bat");
const GRADLEW: FileKey = file_key("gradlew");
const GRADLE_PROPERTIES: FileKey = file_key("gradle.properties");
const BUILD_SCRIPT: FileKey = file_key("build.gradle");
const GITIGNORE: FileKey = file_key(".gitignore");
const GITATTRIBUTES: FileKey = file_key(".gitattributes");
const GRADLE_WRAPPER: FileKey = file_key_binary("gradle/wrapper/gradle-wrapper.jar");
const GRADLE_WRAPPER_PROPERTIES: FileKey = file_key("gradle/wrapper/gradle-wrapper.properties");
const GITHUB_CI: FileKey = file_key(".github/workflows/build.yml");

const FABRIC_MOD_JSON: FileKey = file_key("src/main/resources/fabric.mod.json");
const MIXINS_JSON: FileKey = file_key_modided("src/main/resources/modid.mixins.json");
const ACCESS_WIDENER: FileKey = file_key_modided("src/main/resources/modid.accesswidener");
const ICON: FileKey = file_key_asset("src/main/resources/assets/modid/icon.png", true);

const ENTRYPOINT: FileKey = file_key_modnamed_packaged("src/main/java/packaged/ModName.java");
const CLIENT_ENTRYPOINT: FileKey =
    file_key_modnamed_packaged("src/main/java/packaged/client/ModNameClient.java");
const DATAGEN_ENTRYPOINT: FileKey =
    file_key_modnamed_packaged("src/main/java/packaged/datagen/ModNameDataGenerator.java");

#[derive(Default, Clone)]
pub struct NameModule {
    modid: String,
    modname: String,
    modgroup: String,
    license_holder: String,
}
impl Module for NameModule {
    fn files(&self) -> &'static [crate::key::FileKey] {
        &[
            GRADLE_SETTINGS,
            LICENSE,
            GRADLEW_BAT,
            GRADLEW,
            GRADLE_PROPERTIES,
            BUILD_SCRIPT,
            GITIGNORE,
            GITATTRIBUTES,
            GRADLE_WRAPPER,
            GRADLE_WRAPPER_PROPERTIES,
            GITHUB_CI,
            FABRIC_MOD_JSON,
            MIXINS_JSON,
            ACCESS_WIDENER,
            ICON,
            ENTRYPOINT,
            CLIENT_ENTRYPOINT,
            DATAGEN_ENTRYPOINT,
        ]
    }

    fn export(&self, exports: &mut super::ExportedValues) -> Result<()> {
        if !is_valid_id(&self.modid) {
            bail!("\"{}\" is not a valid Mod ID!", self.modid);
        }
        if !is_valid_group(&self.modgroup) {
            bail!("\"{}\" is not a valid Mod Group!", self.modgroup)
        }
        if self.modname.is_empty() {
            bail!("\"{}\" is not a valid Mod Name!", self.modname)
        }
        exports.set(MOD_NAME, &self.modname);
        exports.set(common::MOD_ID, &self.modid);
        exports.set(MOD_GROUP, &self.modgroup);
        exports.set(LICENSE_HOLDER, &self.license_holder);
        let entrypoint_name = self.modname.clone().replace(" ", "");
        exports.set_owned(common::ENTRYPOINT_NAME, entrypoint_name);
        let pathified_group = self.modgroup.clone().replace('.', "/");
        exports.set_owned(common::PATHIFIED_GROUP, pathified_group);
        let count = self.modgroup.split('.').count();
        let suffixless = self.modgroup.clone().split('.').take(count - 1).join(".");
        exports.set_owned(SUFFIXELESS_GROUP, suffixless);

        //Initialize Appender values
        exports.init(common::ENTRYPOINT_IMPORTS);
        exports.init(common::ENTRYPOINT_INITS);
        exports.init(common::CLIENT_INITS);
        exports.init(common::CLIENT_IMPORTS);
        exports.init(common::DATAGEN_INITS);
        exports.init(common::DATAGEN_IMPORTS);

        Ok(())
    }

    fn show(&mut self, ui: &mut egui::Ui) {
        let l = ui.label("Mod Id").id;
        ui.text_edit_singleline(&mut self.modid).labelled_by(l);
        ui.separator();
        let l = ui.label("Mod Name").id;
        ui.text_edit_singleline(&mut self.modname).labelled_by(l);
        ui.separator();
        let l = ui.label("Mod Group").id;
        ui.text_edit_singleline(&mut self.modgroup).labelled_by(l);
        let l = ui.label("License Holder").id;
        ui.text_edit_singleline(&mut self.license_holder)
            .labelled_by(l);
    }

    fn create_default() -> Self
    where
        Self: Sized,
    {
        NameModule::default()
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
