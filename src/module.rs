pub mod metadata;
pub mod name;
pub mod version;
pub mod tenreg;

use std::collections::{HashMap, HashSet, hash_map::Iter};

use anyhow::Result;
use egui::Ui;
use serde::de;

use crate::key::{ExportValue, FileKey, ValueKey};
#[derive(Debug, Clone, Default)]
pub struct ExportedValues(HashMap<ValueKey, ExportValue>, HashMap<ValueKey, HashSet<ValueKey>>);
impl ExportedValues {
    pub fn set(&mut self, key: ValueKey, value: &str) {
        self.set_owned(key, value.to_owned());
    }
    pub fn set_owned(&mut self, key: ValueKey, value: String) {
        self.0.insert(key, ExportValue::Settable(value));
    }
    pub fn append(&mut self, key: ValueKey, value: &str) {
        self.append_owned(key, value.to_owned());
    }
    pub fn append_dependent(&mut self, key: ValueKey, value: &str, dependencies: Vec<ValueKey>) {
        self.append_owned(key, value.to_owned());
        self.add_dependency(key, dependencies);
    }
    pub fn add_dependency(&mut self, key: ValueKey, dependencies: Vec<ValueKey>) {
        let set = HashSet::from_iter(dependencies);
        let new = self.1.remove(&key).map_or(set.clone(), |old| {
            old.union(&set).map(|vk| *vk).collect()
        });
        self.1.insert(key, new);
    }
    pub fn append_owned(&mut self, key: ValueKey, value: String) {
        let appendable = self.0.get_mut(&key);
        if let Some(ExportValue::Appendable(strings)) = appendable {
            strings.push(value);
        } else {
            self.0
                .insert(key, ExportValue::Appendable(vec![value]));
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
    pub fn canonicalize(&mut self){
        for (dependent, dependencies) in self.1.iter() {
            if let Some(mut inserted) = self.0.remove(dependent) {
                for dependency in dependencies {
                    if dependent == dependency {
                        continue;
                    }
                    if let Some(value) = self.0.get(dependency) {
                        inserted.canonicalize(dependency, value);
                    }
                }
                self.0.insert(*dependent, inserted);
            }
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum OptionalModuleKey {
    Tenreg,
}
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
