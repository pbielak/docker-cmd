/// Container info for TUI
use std::fmt;

#[derive(Clone)]
pub struct ContainerInfo {
    id: String,
    image: String,
    command: String,
    status: String,
}

impl ContainerInfo {
    pub const FIELDS: [&'static str; 4] = ["ID", "IMAGE", "COMMAND", "STATUS"];

    pub fn new(id: String, image: String, command: String, status: String) -> ContainerInfo {
        ContainerInfo {
            id,
            image,
            command,
            status,
        }
    }

    pub fn id(&self) -> &str {
        self.id.as_str()
    }
}

impl fmt::Display for ContainerInfo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}\t{}\t{}\t{}",
            self.id, self.image, self.command, self.status
        )
    }
}
