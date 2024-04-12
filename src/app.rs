use std::error;

/// Application result type.
pub type AppResult<T> = std::result::Result<T, Box<dyn error::Error>>;

/// Application.
#[derive(Debug)]
pub struct App {
    /// Is the application running?
    pub running: bool,
    /// counter
    pub counter: u8,
    pub system_info: SystemInfo
}

#[derive(Default, Debug)]
pub struct SystemInfo {
    pub host_name: String,
    pub total_memory: String,
    pub used_memory: String,
    pub cpus: String,
    pub cpu_usage: String,
    pub monitor_memory: Vec<u64>
}

impl SystemInfo {
    pub fn new(host_name: String, total_memory: String, used_memory: String, cpus: String, cpu_usage: String, monitor_memory: Vec<u64>) -> Self {
        Self { host_name, total_memory, used_memory, cpus, cpu_usage, monitor_memory }
    }
}

impl Default for App {
    fn default() -> Self {
        Self {
            running: true,
            counter: 0,
            system_info: SystemInfo::default()
        }
    }
}

impl App {
    /// Constructs a new instance of [`App`].
    pub fn new() -> Self {
        Self::default()
    }

    /// Handles the tick event of the terminal.
    pub fn tick(&self) {}

    /// Set running to false to quit the application.
    pub fn quit(&mut self) {
        self.running = false;
    }

    pub fn increment_counter(&mut self) {
        if let Some(res) = self.counter.checked_add(1) {
            self.counter = res;
        }
    }

    pub fn decrement_counter(&mut self) {
        if let Some(res) = self.counter.checked_sub(1) {
            self.counter = res;
        }
    }
}
