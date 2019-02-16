/// Container info for TUI
use std::fmt;

use shiplift::rep::Container;

#[derive(Clone)]
pub struct ContainerInfo {
    id: String,
    image: String,
    command: String,
    status: String,
}

impl ContainerInfo {
    const MAX_ID_LENGTH: usize = 12;
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

impl From<Container> for ContainerInfo {
    fn from(c: Container) -> Self {
        ContainerInfo::new(
            c.id[..ContainerInfo::MAX_ID_LENGTH].to_string(),
            c.image,
            c.command,
            c.status,
        )
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
