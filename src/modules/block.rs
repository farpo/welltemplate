use std::path::Path;

use crate::{
    FileTransformer, Module, Template, Transformations,
    modules::{ENTRYPOINT_IMPORT_KEY, ENTRYPOINT_INIT_KEY, version::McVersion, version_of},
    template,
};
pub const BLOCKS: Template = template(
    "block",
    include_str!("../../template/src/main/java/pack/age/block/ModBlocks.java"),
);
pub const BLOCKS_OLD: Template = template(
    "block",
    include_str!("../../template/src/main/java/pack/age/block/ModBlocksOld.java"),
);
pub struct Block(pub String);

impl Module for Block {
    fn write_templates(&self, path: &Path, transformations: &Transformations) {
        let package_path = path.join("src/main/java").join(
            transformations
                .get("``MOD_GROUP``")
                .unwrap()
                .replace(".", "/"),
        );
        let template = match version_of(transformations) {
            McVersion::TWENTYONEONE => BLOCKS_OLD,
            McVersion::TWENTYONEEIGHT => BLOCKS,
        };
        template.write_named(
            &package_path,
            transformations,
            format!("{}Blocks.java", self.0).as_str(),
        );
    }

    fn show_panel(&mut self, ui: &mut egui::Ui) {
        let id = ui.label("Block Class Prefix").id;
        ui.text_edit_singleline(&mut self.0).labelled_by(id);
    }
}
impl FileTransformer for Block {
    fn transform(&self, transformations: &mut Transformations) {
        let inits = transformations.get_mut(ENTRYPOINT_INIT_KEY).unwrap();
        inits.push_str(format!("        {}Blocks.init();\n", self.0).as_str());
        transformations.insert("``BLOCK_PREFIX``", self.0.clone());
        let mut import = String::from("import ");
        let group = transformations.get("``MOD_GROUP``").unwrap().as_str();
        import.push_str(group);
        import.push_str(".block.");
        import.push_str(self.0.as_str());
        import.push_str("Blocks;\n");
        let imports = transformations.get_mut(ENTRYPOINT_IMPORT_KEY).unwrap();
        imports.push_str(import.as_str());
    }
}
