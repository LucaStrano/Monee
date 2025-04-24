use std::fmt::Debug;
use std::result::Result;
use sysinfo::{MemoryRefreshKind, System};

use crate::shared::error::FetchError;

#[derive(Debug)]
pub struct UsageMetrics {
    cpu_usage: f32,
    cpus_usage: Vec<f32>,
    memory_usage: f64,
    swap_usage: f64,
}

/// Fetches the CPU usage of the system.
/// Returns a Result containing the CPU usage as a f32.
/// If the CPU usage is invalid (less than 0.0 or greater than 100.0), it returns a FetchError.
fn fetch_global_cpu(sys: &mut System) -> Result<f32, FetchError> {
    sys.refresh_cpu_usage();
    let cpu_usage = sys.global_cpu_usage();
    if cpu_usage < 0.0 || cpu_usage > 100.0 {
        return Err(FetchError::InvalidValue);
    }
    Ok(cpu_usage)
}

fn fetch_cpus_usage(sys: &mut System) -> Result<Vec<f32>, FetchError> {
    sys.refresh_cpu_usage();
    let mut cpus = Vec::<f32>::new();
    for cpu in sys.cpus() {
        let usage = cpu.cpu_usage();
        if usage < 0.0 || usage > 100.0 {
            return Err(FetchError::InvalidValue);
        }
        cpus.push(usage);
    }
    Ok(cpus)
}

fn fetch_memory_usage(sys: &mut System) -> Result<u64, FetchError>{
    sys.refresh_memory_specifics(MemoryRefreshKind::nothing().with_ram());
    let usage = sys.used_memory();
    if usage > sys.total_memory(){
        return Err(FetchError::InvalidValue);
    }
    Ok(usage)
}

fn fetch_swap_usage(sys: &mut System) -> Result<u64, FetchError>{
    sys.refresh_memory_specifics(MemoryRefreshKind::nothing().with_swap());
    let usage = sys.used_swap();
    if usage > sys.total_swap(){
        return Err(FetchError::InvalidValue);
    }
    Ok(usage)
}



pub fn get_usage_metrics(sys: &mut System, scale_factor: f64) -> Result<UsageMetrics, FetchError> {
    let cpu_usage = fetch_global_cpu(sys)?;
    let cpus_usage = fetch_cpus_usage(sys)?;
    let memory_usage = fetch_memory_usage(sys)?;
    let swap_usage = fetch_swap_usage(sys)?;
    let metrics = UsageMetrics {
        cpu_usage: cpu_usage,
        cpus_usage: cpus_usage,
        memory_usage: memory_usage as f64 /scale_factor,
        swap_usage: swap_usage as f64 /scale_factor,
    };

    return Ok(metrics)
}