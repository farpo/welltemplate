use std::path::Path;

use crate::{
    FileTransformer, Module, Template, Transformations, modules::DATAGEN_INIT_KEY, template,
};
pub const MODEL: Template = template(
    "datagen/ModelGen.java",
    include_str!("../../../templating/src/main/java/pack/age/datagen/ModelGen.java"),
);
const SPAWN_EGG_METHOD: &str =
    "    private void registerSpawnEgg(ItemModelGenerator generator, EntityType<?> entityType){
        if(SpawnEggItem.forEntity(entityType) instanceof SpawnEggItem item){
            generator.register(item, CustomModels.SPAWN_EGG);
        }
    }";
const SPAWN_EGG_FIELD: &str = "        protected static final Model SPAWN_EGG = new Model(Optional.of(Identifier.ofVanilla(\"item/template_spawn_egg\")), Optional.empty());
";
pub struct Model(pub bool);

impl Module for Model {
    fn write_templates(&self, path: &Path, transformations: &Transformations) {
        let package_path = path.join("src/main/java").join(
            transformations
                .get("``MOD_GROUP``")
                .unwrap()
                .replace(".", "/"),
        );
        MODEL.write(&package_path, transformations);
    }

    fn show_panel(&mut self, ui: &mut egui::Ui) {
        //let id = ui.label("Add Spawn Eggs").id;
        ui.toggle_value(&mut self.0, "Spawn Eggs")
        //.labelled_by(id)
        ;
    }
}
impl FileTransformer for Model {
    fn transform(&self, transformations: &mut Transformations) {
        let inits = transformations.get_mut(DATAGEN_INIT_KEY).unwrap();
        inits.push_str("        pack.addProvider(ModelGen::new);\n");
        transformations.insert(
            "``MAYBE_SPAWN_EGG_METHOD``",
            match self.0 {
                true => SPAWN_EGG_METHOD,
                false => "",
            }
            .to_owned(),
        );
        transformations.insert(
            "``MAYBE_SPAWN_EGG_FIELD``",
            match self.0 {
                true => SPAWN_EGG_FIELD,
                false => "",
            }
            .to_owned(),
        );
    }
}
