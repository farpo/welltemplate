use std::path::Path;

use egui::Ui;

use crate::{FileTransformer, Module, Transformations};

pub struct Metadata {
    authors: Vec<String>,
    edited_author: String,
    homepage: String,
    repo: String,
    description: String,
}
impl Default for Metadata {
    fn default() -> Self {
        Metadata {
            authors: vec![],
            edited_author: String::new(),
            homepage: "https://nexusrealms.de".to_owned(),
            repo: "https://github.com/farpo/welltemplate".to_owned(),
            description: "Tell people about your mod!".to_owned(),
        }
    }
}
impl Module for Metadata {
    fn write_templates(&self, _path: &Path, _transformations: &Transformations) {}

    fn show_panel(&mut self, ui: &mut Ui) {
        ui.label("Authors:");
        let i = self.authors.len();
        for i in 0..i {
            ui.horizontal(|ui| {
                ui.text_edit_singleline(self.authors.get_mut(i).unwrap_or(&mut "Ghost".to_owned()));
                if ui.button("Remove").clicked() {
                    self.authors.remove(i);
                }
            });
        }
        ui.horizontal(|ui| {
            ui.text_edit_singleline(&mut self.edited_author);
            if ui.button("Add").clicked() {
                self.authors.push(self.edited_author.clone());
                self.edited_author = String::new();
            }
        });
        ui.separator();
        let id = ui.label("Homepage").id;
        ui.text_edit_singleline(&mut self.homepage).labelled_by(id);
        ui.separator();
        let id = ui.label("Repo").id;
        ui.text_edit_singleline(&mut self.repo).labelled_by(id);
        ui.separator();
        let id = ui.label("Description").id;
        ui.text_edit_singleline(&mut self.description)
            .labelled_by(id);
    }
}
impl FileTransformer for Metadata {
    fn transform(&self, transformations: &mut Transformations) {
        transformations.insert("``HOMEPAGE``", self.homepage.clone());
        transformations.insert("``REPO``", self.repo.clone());
        transformations.insert("``DESCRIPTION``", self.description.clone());
        transformations.insert("``MOD_AUTHORS``", self.authors.join("\",\n\""));
    }
}
