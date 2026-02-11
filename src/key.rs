use std::path::PathBuf;

use crate::module::ExportedValues;

#[derive(Debug, Clone, Copy, Eq, Hash)]
pub struct FileKey(
    pub &'static str,
    Option<fn(&'static str, &ExportedValues) -> String>,
    pub bool,
);
impl FileKey {
    pub fn resolve_path(&self, exports: &ExportedValues) -> PathBuf {
        PathBuf::from(match self.1 {
            Some(fun) => fun(self.0, exports),
            None => self.0.to_owned(),
        })
    }
}
pub const fn file_key(path: &'static str) -> FileKey {
    FileKey(path, None, false)
}
pub const fn file_key_binary(path: &'static str) -> FileKey {
    FileKey(path, None, true)
}
pub const fn file_key_packaged(path: &'static str) -> FileKey {
    file_key_mapped(
        path,
        |path, values| {
            //this will be the pathified package
            path.to_owned()
                .replace("packaged", &values.get(common::PATHIFIED_GROUP).unwrap())
        },
        false,
    )
}
pub const fn file_key_modnamed(path: &'static str) -> FileKey {
    file_key_mapped(
        path,
        |path, values| {
            //this will be the classnameified mod name
            path.to_owned()
                .replace("ModName", &values.get(common::ENTRYPOINT_NAME).unwrap())
        },
        false,
    )
}
pub const fn file_key_modnamed_packaged(path: &'static str) -> FileKey {
    file_key_mapped(
        path,
        |path, values| {
            //this will be the classnameified mod name
            path.to_owned()
                .replace("ModName", &values.get(common::ENTRYPOINT_NAME).unwrap())
                .replace("packaged", &values.get(common::PATHIFIED_GROUP).unwrap())
        },
        false,
    )
}
pub const fn file_key_modided(path: &'static str) -> FileKey {
    file_key_mapped(
        path,
        |path, values| {
            //this will be the raw modid
            path.to_owned()
                .replace("modid", &values.get(common::MOD_ID).unwrap())
        },
        false,
    )
}
pub const fn file_key_asset(path: &'static str, binary: bool) -> FileKey {
    file_key_mapped(
        path,
        |path, values| {
            //this will be the raw modid
            path.to_owned().replace(
                "assets/modid",
                &format!("assets/{}", values.get(common::MOD_ID).unwrap()),
            )
        },
        binary,
    )
}
pub const fn file_key_mapped(
    path: &'static str,
    mapper: fn(&'static str, &ExportedValues) -> String,
    binary: bool,
) -> FileKey {
    FileKey(path, Some(mapper), binary)
}
impl PartialEq for FileKey {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ValueKey(pub(crate) &'static str);
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ExportValue {
    Settable(String),
    Appendable(Vec<String>),
}
impl ExportValue {
    pub fn resolve(&self) -> String {
        match self {
            ExportValue::Settable(string) => string.clone(),
            ExportValue::Appendable(items) => items.clone().join(""),
        }
    }
}
pub mod common {
    use crate::key::ValueKey;

    pub const ENTRYPOINT_INITS: ValueKey = ValueKey("ENTRYPOINT_INITS");
    pub const ENTRYPOINT_IMPORTS: ValueKey = ValueKey("ENTRYPOINT_IMPORTS");
    pub const DATAGEN_INITS: ValueKey = ValueKey("DATAGEN_INITS");
    pub const DATAGEN_IMPORTS: ValueKey = ValueKey("DATAGEN_IMPORTS");
    pub const CLIENT_INITS: ValueKey = ValueKey("ENTRYPOINT_INITS");
    pub const CLIENT_IMPORTS: ValueKey = ValueKey("CLIENT_IMPORTS");
    pub const ENTRYPOINT_NAME: ValueKey = ValueKey("ENTRYPOINT_NAME");
    pub const PATHIFIED_GROUP: ValueKey = ValueKey("PATHIFIED_GROUP");
    pub const MOD_ID: ValueKey = ValueKey("MOD_ID");
}
