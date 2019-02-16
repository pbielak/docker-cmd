/// Implementation for TUI of docker-cmd
use std::io::Write;

use console::Term;
use dialoguer::{theme::ColorfulTheme, Select};
use tabwriter::TabWriter;

use crate::container_info::ContainerInfo;

pub struct TUI {
    header: String,
    containers: Vec<ContainerInfo>,
}

impl TUI {
    pub fn new(containers: &[ContainerInfo]) -> TUI {
        TUI {
            header: ContainerInfo::FIELDS.join("\t").to_string(),
            containers: containers.to_vec(),
        }
    }

    pub fn get_selected_container(self) -> ContainerInfo {
        let (header, containers) = align_tabs(&self.header, &self.containers);
        let container_idx = get_selection(&header, &containers);

        self.containers[container_idx].clone()
    }
}

fn align_tabs(header: &str, container_info: &[ContainerInfo]) -> (String, Vec<String>) {
    let containers: Vec<String> = container_info.iter().map(|ci| ci.to_string()).collect();

    let mut tw = TabWriter::new(vec![]);
    tw.write_all(format!("{}\n", header).as_bytes()).unwrap();
    tw.write_all(containers.join("\n").as_bytes()).unwrap();
    tw.flush().unwrap();

    let aligned: String = String::from_utf8(tw.into_inner().unwrap()).unwrap();

    let parts: Vec<String> = aligned.split('\n').map(String::from).collect();

    let header: String = format!("  {}", parts[0].clone());
    let info: Vec<String> = parts[1..].to_vec();

    (header, info)
}

fn get_selection<T>(header: &str, items: &[T]) -> usize
where
    T: std::fmt::Display,
{
    let term = Term::stdout();

    term.write_line(header).unwrap();
    let selection = Select::with_theme(&ColorfulTheme::default())
        .default(0)
        .items(&items[..])
        .interact_on(&term)
        .unwrap();
    term.clear_last_lines(1).unwrap();

    selection
}
