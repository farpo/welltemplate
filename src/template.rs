use std::{collections::HashMap, fs, path::PathBuf};

use serde::Deserialize;

use crate::{key::FileKey, module::version::VersionModule};

const PATH: &'static str = "./templates/templates.meta.json";

#[derive(Clone, Debug, Deserialize)]
pub struct TemplateDefinition {
    pub name: String,
    inherits: Option<String>,
    recommended_versions: Option<VersionModule>,
}
impl TemplateDefinition {
    pub fn get_recommendations(&self) -> Option<VersionModule> {
        self.recommended_versions.clone()
    }
}
pub(crate) fn load() -> HashMap<String, TemplateDefinition> {
    let file = fs::read_to_string(PATH).unwrap();
    serde_json::from_str(&file).unwrap()
}
pub fn find(
    key: FileKey,
    selected: &String,
    templates: &HashMap<String, TemplateDefinition>,
) -> Option<PathBuf> {
    let mut template_dir = selected.to_owned();
    let key = key.0;
    loop {
        let path = PathBuf::from(format!("templates/{template_dir}/{key}"));
        if path.exists() {
            break Some(path);
        }
        let template = templates.get(&template_dir).unwrap();
        if let Some(t) = template.inherits.clone() {
            template_dir = t;
        } else {
            break None;
        }
    }
}
