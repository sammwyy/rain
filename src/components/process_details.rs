use sysinfo::{Process, ProcessExt, SystemExt, UserExt};
use tui::{
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{Block, Borders, List, ListItem},
};

use crate::state::State;

pub fn process_details(state: &mut State) -> List {
    let process_index = state.selected_process.selected();
    let texts: Vec<String>;

    if process_index.is_some() {
        let process_index = process_index.unwrap();
        let mut process: Option<&Process> = None;
        let mut i = 0;

        for (_k, v) in state.system.processes().iter() {
            if process_index == i {
                process = Some(v);
            }

            i += 1;
        }

        if process.is_some() {
            let process = process.unwrap();

            let name = process.name();
            let cmd = process.cmd();
            let cwd = process.cwd();
            let run_time = process.run_time();
            let uid = process.user_id();
            let status = process.status().to_string();

            let mut username = "SYSTEM";

            if uid.is_some() {
                let user = state.system.get_user_by_id(uid.unwrap()).unwrap();
                username = user.name();
            }

            texts = vec![
                format!("name"),
                format!("{}", name),
                format!(""),
                format!("cmd"),
                format!("{}", cmd.join(" ")),
                format!(""),
                format!("cwd"),
                format!("{}", cwd.as_os_str().to_str().unwrap()),
                format!(""),
                format!("status"),
                format!("{}", status),
                format!(""),
                format!("time"),
                format!("{}", run_time),
                format!(""),
                format!("user"),
                format!("{}", username),
            ];
        } else {
            texts = vec![];
        }
    } else {
        texts = vec![];
    }

    let spans = (0..texts.len()).map(|i| {
        Spans::from(Span::styled(
            texts[i].clone(),
            if i % 3 == 0 {
                Style::default()
                    .fg(Color::LightBlue)
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(Color::White)
            },
        ))
    });

    let items: Vec<ListItem> = spans.map(|span| ListItem::new(span)).collect();
    let component = List::new(items).block(Block::default().borders(Borders::ALL).title("List"));

    return component;
}
