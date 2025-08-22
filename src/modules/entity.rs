use std::path::Path;

use crate::{
    FileTransformer, Module, Template, Transformations,
    modules::{ENTRYPOINT_IMPORT_KEY, ENTRYPOINT_INIT_KEY},
    template,
};
pub const ENTITIES: Template = template(
    "entity",
    include_str!("../../template/src/main/java/pack/age/entity/ModEntities.java"),
);
pub struct Entity(pub String);

impl Module for Entity {
    fn write_templates(&self, path: &Path, transformations: &Transformations) {
        let package_path = path.join("src/main/java").join(
            transformations
                .get("``MOD_GROUP``")
                .unwrap()
                .replace(".", "/"),
        );
        ENTITIES.write_named(
            &package_path,
            transformations,
            format!("{}Entities.java", self.0).as_str(),
        );
    }

    fn show_panel(&mut self, ui: &mut egui::Ui) {
        let id = ui.label("Entity Class Prefix").id;
        ui.text_edit_singleline(&mut self.0).labelled_by(id);
    }
}
impl FileTransformer for Entity {
    fn transform(&self, transformations: &mut Transformations) {
        let inits = transformations.get_mut(ENTRYPOINT_INIT_KEY).unwrap();
        inits.push_str(format!("        {}Entities.init();\n", self.0).as_str());
        transformations.insert("``ENTITY_PREFIX``", self.0.clone());
        let mut import = String::from("import ");
        let group = transformations.get("``MOD_GROUP``").unwrap().as_str();
        import.push_str(group);
        import.push_str(".entity.");
        import.push_str(self.0.as_str());
        import.push_str("Entities;\n");
        let imports = transformations.get_mut(ENTRYPOINT_IMPORT_KEY).unwrap();
        imports.push_str(import.as_str());
    }
}
