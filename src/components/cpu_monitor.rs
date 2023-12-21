use sysinfo::{CpuExt, SystemExt};
use tui::{
    style::{Color, Style},
    text::{Span, Spans},
    widgets::{Block, BorderType, Borders, List, ListItem},
};

use crate::state::State;

pub fn cpu_monitor(state: &mut State) -> List {
    // Misc
    let brand = state.system.global_cpu_info().brand();
    let usage = state.system.global_cpu_info().cpu_usage();
    let cpus = state.system.cpus();

    // let comp = state.system.components().get(0).unwrap();

    let texts = vec![
        format!("Usage: {}%", usage.ceil()),
        format!("Brand: {}", brand),
        format!("Cores: {}", cpus.len()),
    ];

    let spans = (0..texts.len()).map(|i| {
        Spans::from(Span::styled(
            texts[i].clone(),
            Style::default().fg(Color::White),
        ))
    });

    let items: Vec<ListItem> = spans.map(|span| ListItem::new(span)).collect();
    let component = List::new(items).block(
        Block::default()
            .borders(Borders::all())
            .border_type(BorderType::Plain)
            .title("CPU"),
    );
    return component;
}
