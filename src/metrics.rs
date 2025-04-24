use std::fmt::Debug;

use sysinfo::System;

pub struct UsageMetrics {
    cpu_usage: f32,
    cpus_usage: Vec<f32>,
    memory_usage: u64,
    swap_usage: u64,
}

impl Debug for UsageMetrics{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("UsageMetrics")
        .field("cpu_usage", &self.cpu_usage)
            .field("cpus_usage", &self.cpus_usage)
            .field("memory_usage", &self.memory_usage)
            .field("swap_usage", &self.swap_usage)
            .finish()
    }
}

pub fn get_usage_metrics() -> UsageMetrics {
    let mut sys: System = System::new_all();
    sys.refresh_all();
    let mut metrics = UsageMetrics {
        cpu_usage: sys.global_cpu_usage(),
        cpus_usage: Vec::<f32>::new(),
        memory_usage: sys.used_memory(),
        swap_usage: sys.used_swap(),
    };
    for cpu in sys.cpus() {
        metrics.cpus_usage.push(cpu.cpu_usage());
    }
    return metrics
}