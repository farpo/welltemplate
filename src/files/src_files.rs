use std::path::Path;

use crate::{Template, Transformations, template, template_bin};

pub const FABRIC_MOD_JSON: Template = template(
    "src/main/resources/fabric.mod.json",
    include_str!("../../templating/src/main/resources/fabric.mod.json"),
);
pub const MIXINS: Template = template(
    "src/main/resources",
    include_str!("../../templating/src/main/resources/modname.mixins.json"),
);
pub const ICON: Template = template_bin(
    "src/main/resources/assets",
    include_bytes!("../../templating/src/main/resources/assets/modname/icon.png"),
);
pub const ACCESS_WIDENER: Template = template(
    "src/main/resources/",
    include_str!("../../templating/src/main/resources/modname.accesswidener"),
);

pub const ENTRYPOINT: Template = template(
    "",
    include_str!("../../templating/src/main/java/pack/age/Modname.java"),
);
pub const CLIENT_ENTRYPOINT: Template = template(
    "client",
    include_str!("../../templating/src/main/java/pack/age/client/ModnameClient.java"),
);
pub const DATAGEN_ENTRYPOINT: Template = template(
    "datagen",
    include_str!("../../templating/src/main/java/pack/age/datagen/ModnameDataGenerator.java"),
);

pub const EXAMPLE_MIXIN: Template = template(
    "mixin/ExampleMixin.java",
    include_str!("../../templating/src/main/java/pack/age/mixin/ExampleMixin.java"),
);
pub fn write_resources(path: &Path, transformations: &Transformations) {
    FABRIC_MOD_JSON.write(path, transformations);
    let modid = transformations.get("``MOD_ID``").unwrap().as_str();
    MIXINS.write_named(
        path,
        transformations,
        format!("{}.mixins.json", modid).as_str(),
    );
    ICON.write_named(
        path,
        transformations,
        format!("{}/icon.png", modid).as_str(),
    );
    ACCESS_WIDENER.write_named(
        path,
        transformations,
        format!("{}.accesswidener", modid).as_str(),
    );
}

pub fn write_entrypoints(path: &Path, transformations: &Transformations, entrypoint_name: &str) {
    let package_path = path.join("src/main/java").join(
        transformations
            .get("``MOD_GROUP``")
            .unwrap()
            .replace(".", "/"),
    );
    ENTRYPOINT.write_named(
        &package_path,
        transformations,
        format!("{}.java", entrypoint_name).as_str(),
    );
    CLIENT_ENTRYPOINT.write_named(
        &package_path,
        transformations,
        format!("{}Client.java", entrypoint_name).as_str(),
    );
    DATAGEN_ENTRYPOINT.write_named(
        &package_path,
        transformations,
        format!("{}DataGenerator.java", entrypoint_name).as_str(),
    );
    EXAMPLE_MIXIN.write(&package_path, transformations);
}
