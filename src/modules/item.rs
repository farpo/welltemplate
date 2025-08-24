use std::path::Path;

use crate::{
    FileTransformer, Module, Template, Transformations,
    modules::{ENTRYPOINT_IMPORT_KEY, ENTRYPOINT_INIT_KEY, version::McVersion, version_of},
    template,
};
pub const ITEMS: Template = template(
    "item",
    include_str!("../../template/src/main/java/pack/age/item/ModItems.java"),
);
pub const ITEMS_OLD: Template = template(
    "item",
    include_str!("../../template/src/main/java/pack/age/item/ModItemsOld.java"),
);
pub struct Item(pub String);

impl Module for Item {
    fn write_templates(&self, path: &Path, transformations: &Transformations) {
        let package_path = path.join("src/main/java").join(
            transformations
                .get("``MOD_GROUP``")
                .unwrap()
                .replace(".", "/"),
        );
        let template = match version_of(transformations) {
            McVersion::TWENTYONEONE => ITEMS_OLD,
            McVersion::TWENTYONEEIGHT => ITEMS,
        };
        template.write_named(
            &package_path,
            transformations,
            format!("{}Items.java", self.0).as_str(),
        );
    }

    fn show_panel(&mut self, ui: &mut egui::Ui) {
        let id = ui.label("Item Class Prefix").id;
        ui.text_edit_singleline(&mut self.0).labelled_by(id);
    }
}
impl FileTransformer for Item {
    fn transform(&self, transformations: &mut Transformations) {
        let inits = transformations.get_mut(ENTRYPOINT_INIT_KEY).unwrap();
        inits.push_str(format!("        {}Items.init();\n", self.0).as_str());
        transformations.insert("``ITEM_PREFIX``", self.0.clone());
        let mut import = String::from("import ");
        let group = transformations.get("``MOD_GROUP``").unwrap().as_str();
        import.push_str(group);
        import.push_str(".item.");
        import.push_str(self.0.as_str());
        import.push_str("Items;\n");
        let imports = transformations.get_mut(ENTRYPOINT_IMPORT_KEY).unwrap();
        imports.push_str(import.as_str());
    }
}
