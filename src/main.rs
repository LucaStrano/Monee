mod metrics;
use sysinfo::System;
mod shared { pub mod error; }
use clap::{Parser, ValueEnum};
use std::fmt;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Size format to display. can be mb (Megabytes) or gb (Gigabytes).
    /// Default is gb.
    #[arg(short, long, default_value_t = SizeFormat::GB)]
    size: SizeFormat,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, ValueEnum)]
enum SizeFormat {
    MB,
    GB,
}

impl fmt::Display for SizeFormat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SizeFormat::MB => write!(f, "MB"),
            SizeFormat::GB => write!(f, "GB"),
        }
    }
}

fn get_scale_factor(format: SizeFormat) -> f64 {
    match format {
        SizeFormat::MB => 1024.0 * 1024.0,
        SizeFormat::GB => 1024.0 * 1024.0 * 1024.0
    }
}

fn main() {

    let args = Args::parse();
    let scale_factor = get_scale_factor(args.size);

    let mut global_sys: System = System::new_all();
    global_sys.refresh_all();

    match metrics::get_usage_metrics(&mut global_sys, scale_factor){
        Ok(metrics) => println!("Usage Metrics: {:?}", metrics),
        Err(e) => println!("Error fetching metrics: {:?}", e),
    }

}
