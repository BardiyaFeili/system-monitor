use std::{error::Error, fmt};
use std::time::Duration;
use sysinfo::{Disks, System, Networks};

#[derive(Debug)]
pub struct MetricsSnapshot {
    pub cpu_usage_percent: f32,
    pub memory_used_bytes: u64,
    pub memory_total_bytes: u64,
    pub disk_read_bytes: u64,
    pub disk_write_bytes: u64,
    pub net_rx_bytes: u64,
    pub net_tx_bytes: u64,
}

/// Collect metrics with proper refresh for accurate network and disk I/O
pub fn collect_metrics() -> MetricsSnapshot {
    let mut system = System::new();
    let mut disks = Disks::new_with_refreshed_list();
    let mut networks = Networks::new_with_refreshed_list();
    
    // First refresh to establish baseline
    system.refresh_all();
    disks.refresh(false);
    networks.refresh(false);
    
    // Wait for the interval
    std::thread::sleep(Duration::from_millis(5000));
    
    // Second refresh to get deltas
    system.refresh_all();
    disks.refresh(false);
    networks.refresh(false);
    
    let cpu_usage = system.global_cpu_usage();
    
    // Memory
    let memory_used = system.used_memory();
    let memory_total = system.total_memory();
    
    // Disk I/O (sum all disks) - these are now bytes since last refresh
    let (disk_read, disk_write) = disks
        .iter()
        .map(|disk| disk.usage())
        .fold((0, 0), |(read, write), usage| {
            (read + usage.read_bytes, write + usage.written_bytes)
        });
    
    // Network (sum all interfaces) - these are now bytes since last refresh
    let (rx_bytes, tx_bytes) = networks
        .iter()
        .fold((0, 0), |(rx, tx), (_, data)| {
            (rx + data.received(), tx + data.transmitted())
        });
    
    MetricsSnapshot {
        cpu_usage_percent: cpu_usage,
        memory_used_bytes: memory_used,
        memory_total_bytes: memory_total,
        disk_read_bytes: disk_read,
        disk_write_bytes: disk_write,
        net_rx_bytes: rx_bytes,
        net_tx_bytes: tx_bytes,
    }
}

#[derive(Debug)]
pub struct FormattedMetrics {
    pub cpu_usage: String,
    pub memory_used: String,
    pub memory_total: String,
    pub memory_usage_percent: String,
    pub disk_read: String,
    pub disk_write: String,
    pub net_rx: String,
    pub net_tx: String,
}

impl MetricsSnapshot {
    pub fn format(&self) -> FormattedMetrics {
        FormattedMetrics {
            cpu_usage: format!("{:.1}%", self.cpu_usage_percent),
            memory_used: format_bytes(self.memory_used_bytes),
            memory_total: format_bytes(self.memory_total_bytes),
            memory_usage_percent: format!(
                "{:.1}%",
                (self.memory_used_bytes as f64 / self.memory_total_bytes as f64) * 100.0
            ),
            disk_read: format_bytes(self.disk_read_bytes),
            disk_write: format_bytes(self.disk_write_bytes),
            net_rx: format_bytes(self.net_rx_bytes),
            net_tx: format_bytes(self.net_tx_bytes),
        }
    }
}

impl fmt::Display for FormattedMetrics {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "System Metrics:")?;
        writeln!(f, "  CPU Usage:       {}", self.cpu_usage)?;
        writeln!(f, "  Memory:          {} / {} ({})", 
            self.memory_used, self.memory_total, self.memory_usage_percent)?;
        writeln!(f, "  Disk Read:       {}", self.disk_read)?;
        writeln!(f, "  Disk Write:      {}", self.disk_write)?;
        writeln!(f, "  Network RX:      {}", self.net_rx)?;
        writeln!(f, "  Network TX:      {}", self.net_tx)?;
        Ok(())
    }
}

/// Format bytes into human-readable format (B, KB, MB, GB, TB)
pub fn format_bytes(bytes: u64) -> String {
    const UNITS: &[&str] = &["B", "KB", "MB", "GB", "TB", "PB"];
    
    if bytes == 0 {
        return "0 B".to_string();
    }
    
    let bytes_f = bytes as f64;
    let exponent = (bytes_f.log10() / 1000_f64.log10()).floor() as usize;
    let exponent = exponent.min(UNITS.len() - 1);
    
    let value = bytes_f / 1000_f64.powi(exponent as i32);
    let unit = UNITS[exponent];
    
    if value >= 100.0 {
        format!("{:.0} {}", value, unit)
    } else if value >= 10.0 {
        format!("{:.1} {}", value, unit)
    } else {
        format!("{:.2} {}", value, unit)
    }
}

/// Format bytes per second into human-readable speed format
pub fn format_speed(bytes_per_sec: u64) -> String {
    format!("{}/s", format_bytes(bytes_per_sec))
}

/// Format percentage with one decimal place
pub fn format_percent(value: f32) -> String {
    format!("{:.1}%", value)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_bytes() {
        assert_eq!(format_bytes(0), "0 B");
        assert_eq!(format_bytes(500), "500 B");
        assert_eq!(format_bytes(1500), "1.50 KB");
        assert_eq!(format_bytes(1_500_000), "1.50 MB");
        assert_eq!(format_bytes(1_500_000_000), "1.50 GB");
        assert_eq!(format_bytes(15_000_000_000), "15.0 GB");
        assert_eq!(format_bytes(150_000_000_000), "150 GB");
    }

    #[test]
    fn test_format_speed() {
        assert_eq!(format_speed(1_500_000), "1.50 MB/s");
        assert_eq!(format_speed(150_000_000), "150 MB/s");
    }
}

pub fn print_once() -> Result<(), Box<dyn Error>> {
    println!("{:?}", collect_metrics().format());

    Ok(())
}
