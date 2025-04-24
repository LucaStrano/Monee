mod metrics;
use metrics::UsageMetrics;

fn main() {
    let metrics: UsageMetrics = metrics::get_usage_metrics();
    println!("Usage Metrics: {:?}", metrics);
}
