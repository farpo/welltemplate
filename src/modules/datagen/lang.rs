use std::path::Path;

use crate::{
    FileTransformer, Module, Template, Transformations, modules::DATAGEN_INIT_KEY, template,
};
pub const LANG: Template = template(
    "datagen/LangGen.java",
    include_str!("../../../template/src/main/java/pack/age/datagen/LangGen.java"),
);
pub struct Lang;

impl Module for Lang {
    fn write_templates(&self, path: &Path, transformations: &Transformations) {
        let package_path = path.join("src/main/java").join(
            transformations
                .get("``MOD_GROUP``")
                .unwrap()
                .replace(".", "/"),
        );
        LANG.write(&package_path, transformations);
    }

    fn show_panel(&mut self, ui: &mut egui::Ui) {
        ui.label("Nothing to edit here");
    }
}
impl FileTransformer for Lang {
    fn transform(&self, transformations: &mut Transformations) {
        let inits = transformations.get_mut(DATAGEN_INIT_KEY).unwrap();
        inits.push_str("        pack.addProvider(LangGen::new);\n");
    }
}
