use crate::{FileKey, key::{ValueKey, common, file_key_mapped}, module::{Module, OptionalModule}};

#[derive(Clone, Debug, Default)]
pub struct TenregModule {
    version: String,
    blocks: bool,
    items: bool,
    name_prefix: String
}
impl OptionalModule for TenregModule {
    fn key(&self) -> super::OptionalModuleKey {
        super::OptionalModuleKey::Tenreg
    }

    fn is_valid(&self, _tags: &[String]) -> bool {
        true
    }
}
const fn file_key_tenreg_entrypoint(path: &'static str) -> FileKey {
    file_key_mapped(path, |path, exports| {
        path.replace("tenreg/", "").replace("packaged", &exports.get(common::PATHIFIED_GROUP).unwrap()).replace("PREFIX", &exports.get(CLASSNAME_PREFIX).unwrap_or("".to_owned()))
    }, false)
}
const CLASSNAME_PREFIX: ValueKey = ValueKey("TENREG_CLASSNAME_PREFIX");
const BLOCK_ENTRYPOINT: FileKey = file_key_tenreg_entrypoint("src/main/java/packaged/tenreg/block/PREFIXBlocks.java");
const ITEM_ENTRYPOINT: FileKey = file_key_tenreg_entrypoint("src/main/java/packaged/tenreg/item/PREFIXItems.java");


impl Module for TenregModule {
    fn files(&self) -> &'static [crate::FileKey] {
        match (self.blocks ,self.items) {
            (true, true) => &[BLOCK_ENTRYPOINT, ITEM_ENTRYPOINT],
            (true, false) => &[BLOCK_ENTRYPOINT],
            (false, true) => &[ITEM_ENTRYPOINT],
            (false, false) => &[],
        }
    }

    fn export(&self, exports: &mut super::ExportedValues) -> anyhow::Result<()> {
        exports.set(CLASSNAME_PREFIX, &self.name_prefix);
        exports.append(common::REPOSITORIES, "	maven {
		url 'https://gitlab.nexusrealms.de/api/v4/projects/37/packages/maven'
	}\n");
        exports.append(common::DEPENDENCIES, "	modImplementation \"de.nexusrealms:tenreg:${project.tenreg_version}+${project.minecraft_version}\"\n");
        exports.append_owned(common::GRADLE_PROPERTIES, format!("tenreg_version={}", self.version));
        exports.append(common::ENTRYPOINT_STATICS, "	public static final Reg REG = Reg.create(MOD_ID);\n");
        if self.items {
            exports.append_dependent(common::ENTRYPOINT_IMPORTS, "import ``MOD_GROUP``.item.``TENREG_CLASSNAME_PREFIX``Items;\n", vec![common::MOD_GROUP, CLASSNAME_PREFIX]);
            exports.append_dependent(common::ENTRYPOINT_INITS, "		``TENREG_CLASSNAME_PREFIX``Items.init();\n", vec![CLASSNAME_PREFIX]);
        }
        if self.blocks {
            exports.append_dependent(common::ENTRYPOINT_IMPORTS, "import ``MOD_GROUP``.block.``TENREG_CLASSNAME_PREFIX``Blocks;\n", vec![common::MOD_GROUP, CLASSNAME_PREFIX]);
            exports.append_dependent(common::ENTRYPOINT_INITS, "		``TENREG_CLASSNAME_PREFIX``Blocks.init();\n", vec![CLASSNAME_PREFIX]);
        }

        exports.append(common::ENTRYPOINT_IMPORTS, "import de.nexusrealms.tenreg.Reg;\n");
        exports.append(common::ENTRYPOINT_INITS, "		REG.register();\n");
        exports.append_dependent(common::DATAGEN_IMPORTS, "import ``MOD_GROUP``.``ENTRYPOINT_NAME``;\n", vec![common::MOD_GROUP, common::ENTRYPOINT_NAME]);
        exports.append_dependent(common::DATAGEN_INITS, "		``ENTRYPOINT_NAME``.REG.genData(pack);\n", vec![common::ENTRYPOINT_NAME]);
        Ok(())
    }

    fn show(&mut self, ui: &mut egui::Ui) {
        let id = ui.label("Tenreg version").id;
        ui.text_edit_singleline(&mut self.version).labelled_by(id);
        ui.separator();
        let id = ui.label("Class name prefix").id;
        ui.text_edit_singleline(&mut self.name_prefix).labelled_by(id);
        ui.checkbox(&mut self.blocks, "Blocks");
        ui.checkbox(&mut self.items, "Items");
    }

    fn create_default() -> Self
    where
        Self: Sized {
        TenregModule::default()
    }
}