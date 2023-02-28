use sysinfo::{ProcessExt, SystemExt};
use tui::{
    layout::Constraint,
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, Cell, Row, Table},
};

use crate::state::State;

pub fn process_list(state: &mut State) -> Table {
    let selected_style = Style::default().add_modifier(Modifier::REVERSED);

    fn get_style_by_percent(value: u64, max: u64) -> Style {
        let div: f64 = value as f64 / max as f64;
        let percent = div * 100.0;

        if percent > 75.0 {
            return Style::default().fg(Color::Red);
        } else if percent > 50.0 {
            return Style::default().fg(Color::LightRed);
        } else if percent > 25.0 {
            return Style::default().fg(Color::Yellow);
        } else if percent > 5.0 {
            return Style::default().fg(Color::LightYellow);
        } else if percent > 1.0 {
            return Style::default().fg(Color::LightGreen);
        } else if percent > 0.1 {
            return Style::default().fg(Color::Green);
        } else {
            return Style::default().fg(Color::Gray);
        }
    }

    let header_cells = ["Name", "PID", "Memory", "CPU", "Disk"]
        .iter()
        .map(|h| Cell::from(*h).style(Style::default().fg(Color::LightMagenta)));

    let header = Row::new(header_cells).height(1).bottom_margin(1);

    let rows = state.system.processes().iter().map(|item| {
        let process = item.1;

        let name = process.name();
        let pid = item.0;
        let ram_usage = (process.memory() as f64) / 1024.0 / 1024.0;
        let cpu_usage = process.cpu_usage() / state.system.cpus().len() as f32;
        let disk_usage = (process.disk_usage().read_bytes + process.disk_usage().written_bytes)
            as f64
            / 1024.0
            / 1024.0;

        let data = &mut vec![
            name.to_string(),
            pid.to_string(),
            format!("{ram_usage:.1} MB"),
            format!("{cpu_usage:.1}%"),
            format!("{disk_usage:.1} MB/S"),
        ];

        let styles = &mut vec![
            Style::default().fg(Color::LightCyan),
            Style::default().fg(Color::White),
            get_style_by_percent(ram_usage as _, state.system.total_memory() / 1024 / 1024),
            get_style_by_percent(cpu_usage as _, 100),
            get_style_by_percent(disk_usage as _, 100),
        ];

        let cells = (0..5).map(move |i| Cell::from(data[i].clone()).style(styles[i]));
        Row::new(cells).height(1)
    });

    let component = Table::new(rows)
        .header(header)
        .block(Block::default().borders(Borders::ALL).title("Process List"))
        .highlight_style(selected_style)
        .widths(&[
            Constraint::Percentage(40),
            Constraint::Percentage(15),
            Constraint::Percentage(15),
            Constraint::Percentage(15),
            Constraint::Percentage(15),
        ]);

    return component;
}
