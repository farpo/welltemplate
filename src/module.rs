pub mod metadata;
pub mod name;
pub mod version;

use std::collections::{HashMap, hash_map::Iter};

use anyhow::Result;
use egui::Ui;

use crate::key::{ExportValue, FileKey, ValueKey};
#[derive(Debug, Clone, Default)]
pub struct ExportedValues(HashMap<ValueKey, ExportValue>);
impl ExportedValues {
    pub fn set(&mut self, key: ValueKey, value: &str) {
        self.set_owned(key, value.to_owned());
    }
    pub fn set_owned(&mut self, key: ValueKey, value: String) {
        self.0.insert(key, ExportValue::Settable(value));
    }
    pub fn append(&mut self, key: ValueKey, value: &str) {
        let appendable = self.0.get_mut(&key);
        if let Some(ExportValue::Appendable(strings)) = appendable {
            strings.push(value.to_owned());
        } else {
            self.0
                .insert(key, ExportValue::Appendable(vec![value.to_owned()]));
        }
    }
    pub fn init(&mut self, key: ValueKey) {
        self.0.insert(key, ExportValue::Appendable(vec![]));
    }
    pub fn get(&self, key: ValueKey) -> Option<String> {
        self.0.get(&key).map(ExportValue::resolve)
    }
    pub fn iter(&self) -> Iter<'_, ValueKey, ExportValue> {
        self.0.iter()
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum OptionalModuleKey {}
pub trait Module {
    fn files(&self) -> &'static [FileKey];
    fn export(&self, exports: &mut ExportedValues) -> Result<()>;
    fn show(&mut self, ui: &mut Ui);
    //The normal default trait is not dyn-compatible so im making my own one
    fn create_default() -> Self
    where
        Self: Sized;
}
pub trait OptionalModule: Module {
    fn key(&self) -> OptionalModuleKey;
    fn is_valid(&self, tags: &[String]) -> bool;
}
