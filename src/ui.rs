use tui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout},
    text::Span,
    widgets::{Block, BorderType, Borders},
    Frame,
};

use crate::{app::App, components::process_list::process_list};

pub fn ui<B: Backend>(f: &mut Frame<B>, app: &mut App) {
    let size = f.size();

    // Main block
    let block = Block::default();
    f.render_widget(block, size);

    // Layout
    let main_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(20), Constraint::Percentage(80)].as_ref())
        .split(f.size());

    let top_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .margin(1)
        .constraints(
            [
                Constraint::Percentage(33),
                Constraint::Percentage(33),
                Constraint::Percentage(33),
            ]
            .as_ref(),
        )
        .split(main_chunks[0]);

    let bottom_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(75), Constraint::Percentage(25)].as_ref())
        .split(main_chunks[1]);

    let top_left = top_chunks[0];
    let top_center = top_chunks[1];
    let top_right = top_chunks[2];
    let bottom_left = bottom_chunks[0];
    let bottom_right = bottom_chunks[1];

    // System Monitor
    let system_monitor = Block::default()
        .borders(Borders::all())
        .border_type(BorderType::Plain)
        .title(vec![Span::from("System")]);

    f.render_widget(system_monitor, top_left);

    // CPU Monitor
    let cpu_monitor_area = Block::default()
        .borders(Borders::all())
        .border_type(BorderType::Plain)
        .title(vec![Span::from("CPU")]);

    f.render_widget(cpu_monitor_area, top_center);

    // RAM Monitor
    let ram_monitor_area = Block::default()
        .borders(Borders::all())
        .border_type(BorderType::Plain)
        .title(vec![Span::from("Memory")]);
    f.render_widget(ram_monitor_area, top_right);

    // Proccess list
    let state = &mut app.state.selected_process.clone();
    f.render_stateful_widget(process_list(&mut app.state), bottom_left, state);

    // Process detail
    let process_detail_area = Block::default()
        .borders(Borders::all())
        .border_type(BorderType::Plain)
        .title(vec![Span::from("Process Details")]);
    f.render_widget(process_detail_area, bottom_right);
}
