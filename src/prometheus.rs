use crate::models::ContainerStats;

#[allow(dead_code)]
pub fn json_to_prometheus(container_stats: Vec<ContainerStats>) -> String {
    let mut output = String::new();

    // Add HELP and TYPE headers
    output.push_str(
        "# HELP container_memory_usage_bytes Memory usage in \
         bytes\n",
    );
    output.push_str("# TYPE container_memory_usage_bytes gauge\n");
    output.push_str(
        "# HELP container_memory_usage_percent Memory usage \
         percentage\n",
    );
    output.push_str("# TYPE container_memory_usage_percent gauge\n");
    output.push_str(
        "# HELP container_cpu_usage_percent CPU usage \
         percentage\n",
    );
    output.push_str("# TYPE container_cpu_usage_percent gauge\n");

    for stats in container_stats {
        let container_name = &stats.name;

        // Calculate memory usage percentage
        let cache = stats.memory_stats.stats.file as f64;
        let used_memory = stats.memory_stats.usage as f64 - cache;
        let available_memory = stats.memory_stats.limit as f64;
        let memory_percent = if available_memory > 0.0 {
            (used_memory / available_memory) * 100.0
        } else {
            0.0
        };

        // Calculate CPU usage percentage
        let cpu_delta = stats.cpu_stats.cpu_usage.total_usage
            - stats.precpu_stats.cpu_usage.total_usage;
        let system_cpu_delta = stats.cpu_stats.system_cpu_usage.unwrap_or(0)
            - stats.precpu_stats.system_cpu_usage.unwrap_or(0);
        let number_cpus = stats.cpu_stats.online_cpus.unwrap_or(0) as f64;

        let cpu_percent = if system_cpu_delta > 0 {
            (cpu_delta as f64 / system_cpu_delta as f64) * number_cpus * 100.0
        } else {
            0.0
        };

        output.push_str(&format!(
            "container_memory_usage_bytes{{\
             container_name=\"{}\"}} {}\n",
            container_name, used_memory as u64
        ));
        output.push_str(&format!(
            "container_memory_usage_percent{{\
             container_name=\"{}\"}} {:.2}\n",
            container_name, memory_percent
        ));
        output.push_str(&format!(
            "container_cpu_usage_percent{{\
             container_name=\"{}\"}} {:.2}\n",
            container_name, cpu_percent
        ));
    }

    output
}
