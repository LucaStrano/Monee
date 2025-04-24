mod metrics;
use sysinfo::System;
mod shared { pub mod error; }

fn main() {

    let mut global_sys: System = System::new_all();
    global_sys.refresh_all();

    match metrics::get_usage_metrics(&mut global_sys){
        Ok(metrics) => println!("Usage Metrics: {:?}", metrics),
        Err(e) => println!("Error fetching metrics: {:?}", e),
    }

}
