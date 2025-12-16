use sysinfo::{ProcessRefreshKind, RefreshKind, System};

#[derive(PartialEq)]
pub enum SortBy {
    Pid,
    Name,
    Cpu,
    Memory,
}

pub struct App {
    pub system: System,
    pub running: bool,
    pub state: ratatui::widgets::TableState,
    pub processes: Vec<(sysinfo::Pid, String, f32, u64)>,
    pub sort_by: SortBy,
}

impl App {
    pub fn new() -> Self {
        let mut system = System::new_all();
        system.refresh_all();

        let mut app = Self {
            system,
            running: true,
            state: ratatui::widgets::TableState::default(),
            processes: Vec::new(),
            sort_by: SortBy::Memory,
        };
        app.update_processes_list();
        app
    }

    pub fn on_tick(&mut self) {
        self.system.refresh_specifics(
            RefreshKind::new().with_processes(ProcessRefreshKind::everything()),
        );
        self.update_processes_list();
    }

    fn update_processes_list(&mut self) {
        self.processes = self.system.processes().iter().map(|(pid, process)| {
            (
                *pid,
                process.name().to_string(),
                process.cpu_usage(),
                process.memory(),
            )
        }).collect();

        self.sort();
    }

    fn sort(&mut self) {
        match self.sort_by {
            SortBy::Pid => self.processes.sort_by(|a, b| b.0.cmp(&a.0)),
            SortBy::Name => self.processes.sort_by(|a, b| a.1.cmp(&b.1)),
            SortBy::Cpu => self.processes.sort_by(|a, b| b.2.partial_cmp(&a.2).unwrap()),
            SortBy::Memory => self.processes.sort_by(|a, b| b.3.cmp(&a.3)),
        }
    }

    pub fn toggle_sort(&mut self) {
        self.sort_by = match self.sort_by {
            SortBy::Cpu => SortBy::Memory,
            SortBy::Memory => SortBy::Pid,
            SortBy::Pid => SortBy::Name,
            SortBy::Name => SortBy::Cpu,
        };
        self.sort();
    }

    pub fn next(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i >= self.processes.len().saturating_sub(1) {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }

    pub fn previous(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i == 0 {
                    self.processes.len().saturating_sub(1)
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }

    pub fn kill_selected_process(&mut self) {
        if let Some(index) = self.state.selected() {
            if let Some((pid, _, _, _)) = self.processes.get(index) {
                if let Some(process) = self.system.process(*pid) {
                    process.kill();
                }
            }
        }
    }
}