use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use std::{
    io,
    time::{Duration, Instant},
};
use sysinfo::{System, SystemExt};
use tui::{backend::Backend, Terminal};

use crate::{state::State, ui::ui};

pub struct App {
    pub state: State,
}

impl App {
    pub fn new() -> App {
        let mut system = System::new_all();
        system.refresh_all();

        let state = State::new(system);

        App { state }
    }

    pub fn on_tick(&mut self) {
        self.state.system.refresh_all();
    }

    pub fn run<B: Backend>(
        &mut self,
        terminal: &mut Terminal<B>,
        tick_rate: Duration,
    ) -> io::Result<()> {
        let mut last_tick = Instant::now();
        loop {
            terminal.draw(|f| ui(f, self))?;

            let timeout = tick_rate
                .checked_sub(last_tick.elapsed())
                .unwrap_or_else(|| Duration::from_secs(0));
            if crossterm::event::poll(timeout)? {
                if let Event::Key(key) = event::read()? {
                    if key.kind == KeyEventKind::Press {
                        match key.code {
                            KeyCode::Char('q') => return Ok(()),
                            KeyCode::Left => self.state.unselect_process(),
                            KeyCode::Down => self.state.next_process(),
                            KeyCode::Up => self.state.previous_process(),
                            _ => {}
                        }
                    }
                }
            }
            if last_tick.elapsed() >= tick_rate {
                self.on_tick();
                last_tick = Instant::now();
            }
        }
    }
}
