use sysinfo::{System, SystemExt};
use tui::widgets::TableState;

pub struct State {
    pub selected_process: TableState,
    pub system: System,
}

impl State {
    pub fn new(system: System) -> State {
        State {
            selected_process: TableState::default(),
            system,
        }
    }

    pub fn next_process(&mut self) {
        let i = match self.selected_process.selected() {
            Some(i) => {
                if i >= self.system.processes().len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.selected_process.select(Some(i));
    }

    pub fn previous_process(&mut self) {
        let i = match self.selected_process.selected() {
            Some(i) => {
                if i == 0 {
                    self.system.processes().len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.selected_process.select(Some(i));
    }

    pub fn unselect_process(&mut self) {
        self.selected_process.select(None);
    }
}
