use std::path::Path;

use crate::{
    FileTransformer, Module, Template, Transformations, modules::DATAGEN_INIT_KEY, template,
};
pub const TAG: Template = template(
    "datagen/TagGen.java",
    include_str!("../../../template/src/main/java/pack/age/datagen/TagGen.java"),
);
pub struct Tag;

impl Module for Tag {
    fn write_templates(&self, path: &Path, transformations: &Transformations) {
        let package_path = path.join("src/main/java").join(
            transformations
                .get("``MOD_GROUP``")
                .unwrap()
                .replace(".", "/"),
        );
        TAG.write(&package_path, transformations);
    }

    fn show_panel(&mut self, ui: &mut egui::Ui) {
        ui.label("Nothing to edit here");
        ui.label("This adds block, item, and entity tag generation");
        ui.label("Maybe in the future ill make you able set which ones you want");
    }
}
impl FileTransformer for Tag {
    fn transform(&self, transformations: &mut Transformations) {
        let inits = transformations.get_mut(DATAGEN_INIT_KEY).unwrap();
        inits.push_str("        pack.addProvider(TagGen.ItemGen::new);\n");
        inits.push_str("        pack.addProvider(TagGen.BlockGen::new);\n");
        inits.push_str("        pack.addProvider(TagGen.EntityGen::new);\n");
    }
}
