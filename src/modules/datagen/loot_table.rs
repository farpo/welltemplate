use std::path::Path;

use crate::{
    FileTransformer, Module, Template, Transformations, modules::DATAGEN_INIT_KEY, template,
};
pub const LOOT_TABLE: Template = template(
    "datagen/LootTableGen.java",
    include_str!("../../../templating/src/main/java/pack/age/datagen/LootTableGen.java"),
);
pub struct LootTable;

impl Module for LootTable {
    fn write_templates(&self, path: &Path, transformations: &Transformations) {
        let package_path = path.join("src/main/java").join(
            transformations
                .get("``MOD_GROUP``")
                .unwrap()
                .replace(".", "/"),
        );
        LOOT_TABLE.write(&package_path, transformations);
    }

    fn show_panel(&mut self, ui: &mut egui::Ui) {
        ui.label("Nothing to edit here");
        ui.label("This adds block and entity loot table generation");
        ui.label("Maybe in the future ill make you able set which ones you want");
        ui.label("And add more of them");
    }
}
impl FileTransformer for LootTable {
    fn transform(&self, transformations: &mut Transformations) {
        let inits = transformations.get_mut(DATAGEN_INIT_KEY).unwrap();
        inits.push_str("        pack.addProvider(LootTableGen.Entity::new);\n");
        inits.push_str("        pack.addProvider(LootTableGen.Block::new);\n");
    }
}
