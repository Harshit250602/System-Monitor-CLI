use sysinfo::{System, Disks, Networks};

pub struct ProcessInfo {
    pub pid: u32,
    pub name: String,
    pub cpu: f32,
    pub memory: u64,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Tab {
    Overview,
    Processes,
    Network,
}

impl Tab {
    pub fn next(&self) -> Self {
        match self {
            Tab::Overview => Tab::Processes,
            Tab::Processes => Tab::Network,
            Tab::Network => Tab::Overview,
        }
    }

    pub fn previous(&self) -> Self {
        match self {
            Tab::Overview => Tab::Network,
            Tab::Processes => Tab::Overview,
            Tab::Network => Tab::Processes,
        }
    }
}

pub struct App {
    pub system: System,
    pub disks: Disks,
    pub networks: Networks,
    pub processes: Vec<ProcessInfo>,
    pub network_history: Vec<(f64, f64)>,
    pub current_tab: Tab,
    pub should_quit: bool,
}

impl App {
    pub fn new() -> Self {
        let mut system = System::new_all();
        system.refresh_all();
        let disks = Disks::new_with_refreshed_list();
        let networks = Networks::new_with_refreshed_list();
        Self {
            system,
            disks,
            networks,
            processes: Vec::new(),
            network_history: Vec::new(),
            current_tab: Tab::Overview,
            should_quit: false,
        }
    }

    pub fn on_tick(&mut self) {
        self.system.refresh_all();
        self.disks.refresh(true);
        self.networks.refresh(true);
        if self.current_tab == Tab::Processes {
            self.update_processes();
        }
        if self.current_tab == Tab::Network {
            self.update_network();
        }
    }

    fn update_network(&mut self) {
        let mut total_rx = 0;
        let mut total_tx = 0;
        for (_interface_name, network) in &self.networks {
            total_rx += network.received();
            total_tx += network.transmitted();
        }
        self.network_history.push((total_rx as f64, total_tx as f64));
        if self.network_history.len() > 100 {
            self.network_history.remove(0);
        }
    }

    fn update_processes(&mut self) {
        let mut processes: Vec<ProcessInfo> = self.system.processes().iter().map(|(pid, process)| {
            ProcessInfo {
                pid: pid.as_u32(),
                name: process.name().to_string_lossy().to_string(),
                cpu: process.cpu_usage(),
                memory: process.memory(),
            }
        }).collect();
        
        // Sort by CPU usage descending
        processes.sort_by(|a, b| b.cpu.partial_cmp(&a.cpu).unwrap_or(std::cmp::Ordering::Equal));
        
        self.processes = processes;
    }

    pub fn next_tab(&mut self) {
        self.current_tab = self.current_tab.next();
    }

    pub fn previous_tab(&mut self) {
        self.current_tab = self.current_tab.previous();
    }

    pub fn quit(&mut self) {
        self.should_quit = true;
    }
}
