use sysinfo::System;

#[derive(Debug, Clone, Default)]
pub struct Diagnostics {
    system: Option<System>,
    pub cpu_percent: f32,
    pub memory_used_mb: u64,
    pub memory_total_mb: u64,
}

impl Diagnostics {
    pub fn new() -> Self {
        let mut this = Self { system: Some(System::new_all()), ..Default::default() };
        this.refresh();
        this
    }

    pub fn refresh(&mut self) {
        let Some(system) = self.system.as_mut() else { return; };
        system.refresh_cpu_usage();
        system.refresh_memory();
        self.cpu_percent = system.global_cpu_usage().clamp(0.0, 100.0);
        self.memory_used_mb = system.used_memory() / (1024 * 1024);
        self.memory_total_mb = system.total_memory() / (1024 * 1024);
    }
}
