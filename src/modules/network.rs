use std::path::Path;

use crate::{
    FileTransformer, Module, Template, Transformations,
    modules::{ENTRYPOINT_IMPORT_KEY, ENTRYPOINT_INIT_KEY},
    template,
};
pub const PACKETS: Template = template(
    "network",
    include_str!("../../templating/src/main/java/pack/age/network/ModPackets.java"),
);
pub const RECEIVER_PACKET: Template = template(
    "network/ReceiverPacket.java",
    include_str!("../../templating/src/main/java/pack/age/network/ReceiverPacket.java"),
);
pub struct Network(pub String);

impl Module for Network {
    fn write_templates(&self, path: &Path, transformations: &Transformations) {
        let package_path = path.join("src/main/java").join(
            transformations
                .get("``MOD_GROUP``")
                .unwrap()
                .replace(".", "/"),
        );
        PACKETS.write_named(
            &package_path,
            transformations,
            format!("{}Packets.java", self.0).as_str(),
        );
        RECEIVER_PACKET.write(&package_path, transformations);
    }

    fn show_panel(&mut self, ui: &mut egui::Ui) {
        let id = ui.label("Packet Class Prefix").id;
        ui.text_edit_singleline(&mut self.0).labelled_by(id);
    }
}
impl FileTransformer for Network {
    fn transform(&self, transformations: &mut Transformations) {
        let inits = transformations.get_mut(ENTRYPOINT_INIT_KEY).unwrap();
        inits.push_str(format!("        {}Packets.init();\n", self.0).as_str());
        transformations.insert("``PACKET_PREFIX``", self.0.clone());
        let mut import = String::from("import ");
        let group = transformations.get("``MOD_GROUP``").unwrap().as_str();
        import.push_str(group);
        import.push_str(".network.");
        import.push_str(self.0.as_str());
        import.push_str("Packets;\n");
        let imports = transformations.get_mut(ENTRYPOINT_IMPORT_KEY).unwrap();
        imports.push_str(import.as_str());
    }
}
