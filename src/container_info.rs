/// Container info for TUI
use std::clone;
use std::fmt;

use shiplift::rep::Container;

pub struct ContainerInfo {
    pub id: String,
    image: String,
    command: String,
    status: String,
}

impl ContainerInfo {
    pub const FIELDS: [&'static str; 4] = ["ID", "IMAGE", "COMMAND", "STATUS"];

    pub fn new(c: &Container) -> ContainerInfo {
        ContainerInfo {
            id: c.id[..12].to_string(),
            image: c.image.clone(),
            command: c.command.clone(),
            status: c.status.clone(),
        }
    }
}

impl clone::Clone for ContainerInfo {
    fn clone(&self) -> ContainerInfo {
        ContainerInfo {
            id: self.id.clone(),
            image: self.image.clone(),
            command: self.command.clone(),
            status: self.status.clone(),
        }
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
