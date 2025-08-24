use crate::{Transformations, modules::version::McVersion};

pub mod block;
pub mod datagen;
pub mod entity;
pub mod item;
pub mod metadata;
pub mod network;
pub mod version;
pub const ENTRYPOINT_INIT_KEY: &str = "``ENTRYPOINT_INITS``";
pub const ENTRYPOINT_IMPORT_KEY: &str = "``ENTRYPOINT_IMPORTS``";
pub const DATAGEN_INIT_KEY: &str = "``DATAGEN_INITS``";

fn version_of(transformations: &Transformations) -> McVersion {
    transformations
        .get("``MC_VERSION``")
        .unwrap_or(&"".to_owned())
        .into()
}
