use crate::{key::ValueKey, module::Module};

const MOD_AUTHORS: ValueKey = ValueKey("MOD_AUTHORS");
const DESCRIPTION: ValueKey = ValueKey("DESCRIPTION");
const HOMEPAGE: ValueKey = ValueKey("HOMEPAGE");
const REPO: ValueKey = ValueKey("REPO");

#[derive(Clone, Debug)]
pub struct MetadataModule {
    authors: Vec<String>,
    edited_author: String,
    description: String,
    homepage: String,
    repo: String,
}
impl Module for MetadataModule {
    fn files(&self) -> &'static [crate::key::FileKey] {
        &[]
    }

    fn export(&self, exports: &mut super::ExportedValues) -> anyhow::Result<()> {
        exports.set_owned(MOD_AUTHORS, self.authors.clone().join("\",\n        \""));
        exports.set(DESCRIPTION, &self.description);
        exports.set(HOMEPAGE, &self.homepage);
        exports.set(REPO, &self.repo);
        Ok(())
    }

    fn show(&mut self, ui: &mut egui::Ui) {
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

    fn create_default() -> Self
    where
        Self: Sized,
    {
        MetadataModule {
            authors: vec![],
            edited_author: String::new(),
            description: "Tell people about your mod!".to_owned(),
            homepage: "https://nexusrealms.de".to_owned(),
            repo: "https://github.com/farpo/welltemplate".to_owned(),
        }
    }
}
