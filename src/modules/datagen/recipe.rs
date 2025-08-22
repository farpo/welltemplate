use std::path::Path;

use crate::{
    FileTransformer, Module, Template, Transformations, modules::DATAGEN_INIT_KEY, template,
};
pub const RECIPE: Template = template(
    "datagen/RecipeGen.java",
    include_str!("../../../templating/src/main/java/pack/age/datagen/RecipeGen.java"),
);
pub struct Recipe;

impl Module for Recipe {
    fn write_templates(&self, path: &Path, transformations: &Transformations) {
        let package_path = path.join("src/main/java").join(
            transformations
                .get("``MOD_GROUP``")
                .unwrap()
                .replace(".", "/"),
        );
        RECIPE.write(&package_path, transformations);
    }

    fn show_panel(&mut self, ui: &mut egui::Ui) {
        ui.label("Nothing to edit here");
    }
}
impl FileTransformer for Recipe {
    fn transform(&self, transformations: &mut Transformations) {
        let inits = transformations.get_mut(DATAGEN_INIT_KEY).unwrap();
        inits.push_str("        pack.addProvider(RecipeGen::new);\n");
    }
}