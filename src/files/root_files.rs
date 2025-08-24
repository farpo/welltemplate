use std::path::Path;

use crate::{Template, Transformations, template, template_bin};

pub const GRADLE_SETTINGS: Template = template(
    "settings.gradle",
    include_str!("../../template/settings.gradle"),
);
pub const LICENSE: Template = template("LICENSE", include_str!("../../template/LICENSE"));
pub const GRADLEW: Template = template("gradlew", include_str!("../../template/gradlew"));
pub const GRADLEW_BAT: Template =
    template("gradlew.bat", include_str!("../../template/gradlew.bat"));
pub const GRADLE_PROPERTIES: Template = template(
    "gradle.properties",
    include_str!("../../template/gradle.properties"),
);
pub const BUILD_GRADLE: Template =
    template("build.gradle", include_str!("../../template/build.gradle"));
pub const GITATTRIBUTES: Template = template(
    ".gitattributes",
    include_str!("../../template/.gitattributes"),
);
pub const GITINGNORE: Template = template(".gitignore", include_str!("../../template/.gitignore"));

pub const GRADLE_WRAPPER: Template = template_bin(
    "gradle/wrapper/gradle-wrapper.jar",
    include_bytes!("../../template/gradle/wrapper/gradle-wrapper.jar"),
);
pub const GRADLE_WRAPPER_PROPERTIES: Template = template(
    "gradle/wrapper/gradle-wrapper.properties",
    include_str!("../../template/gradle/wrapper/gradle-wrapper.properties"),
);

pub const BUILD_WORKFLOW: Template = template(
    ".github/workflows/build.yml",
    include_str!("../../template/.github/workflows/build.yml"),
);

fn get_templates() -> Vec<Template> {
    vec![
        GRADLE_SETTINGS,
        LICENSE,
        GRADLEW,
        GRADLEW_BAT,
        GRADLE_PROPERTIES,
        BUILD_GRADLE,
        GITATTRIBUTES,
        GITINGNORE,
        GRADLE_WRAPPER,
        GRADLE_WRAPPER_PROPERTIES,
        BUILD_WORKFLOW,
    ]
}

pub fn write(path: &Path, transformations: &Transformations) {
    for template in get_templates() {
        template.write(path, transformations);
    }
}
