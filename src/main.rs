use anyhow::Result;
use chrono::DateTime;
use clap::Parser;
use inline_colorization::*;
use std::fs;
use std::time::UNIX_EPOCH;

/// Command-line arguments parser
#[derive(Parser)]
#[clap(about = "A simple ls implementation in Rust", version = "1.0")]
struct Args {
    /// Show hidden files
    #[clap(short, long)]
    all: bool,
    /// Sort by modified time
    #[clap(short, long)]
    time: bool,
    /// Sort by modified size
    #[clap(short, long)]
    size: bool,
    /// reverse sort
    #[clap(short, long)]
    reverse: bool,
}

fn main() -> Result<()> {
    let args = Args::parse();

    let mut entries = vec![];

    for e in fs::read_dir(".")? {
        entries.push(e?);
    }

    if args.time {
        entries.sort_by(|a, b| {
            a.metadata()
                .unwrap()
                .modified()
                .unwrap()
                .cmp(&b.metadata().unwrap().modified().unwrap())
        });
    }

    if args.size {
        entries.sort_by(|a, other| {
            a.metadata()
                .unwrap()
                .len()
                .cmp(&other.metadata().unwrap().len())
        });
    }

    if args.reverse {
        entries.reverse();
    }

    //Enable virtual term
    colour::red!("");

    for entry in entries {
        let file_name = entry.file_name();
        let file_name = file_name.to_string_lossy();

        // Skip hidden files unless --all is specified
        if !args.all && file_name.starts_with('.') {
            continue;
        }

        let metadata = match entry.metadata() {
            Ok(meta) => meta,
            Err(e) => {
                eprintln!("Error fetching metadata: {}", e);
                continue;
            }
        };

        // File size
        let size = metadata.len();

        // File modification time
        let mod_time = match metadata.modified() {
            Ok(time) => match time.duration_since(UNIX_EPOCH) {
                Ok(duration) => duration.as_secs(),
                Err(_) => 0,
            },
            Err(_) => 0,
        };
        let mod_time = format_system_time(mod_time);

        // File type and coloring
        let colored_name = if metadata.is_dir() {
            format!("{style_bold}{color_blue}{file_name}{color_reset}{style_reset}")
        // Directories in blue
        } else {
            file_name.to_string() // Regular files
        };

        println!(
            "{:<10} {:<20} {}",
            human_readable_size(size),
            mod_time,
            colored_name
        );
    }

    Ok(())
}

/// Format a UNIX timestamp to a readable date
fn format_system_time(timestamp: u64) -> String {
    let datetime = DateTime::from_timestamp(timestamp as i64, 0);
    datetime
        .map(|dt| dt.format("%Y-%m-%d %H:%M:%S").to_string())
        .unwrap_or_else(|| "N/A".to_string())
}

/// Converts a size in bytes to a human-readable string (e.g., KB, MB, GB)
fn human_readable_size(bytes: u64) -> String {
    const UNITS: [&str; 5] = ["B", "KB", "MB", "GB", "TB"];
    let mut size = bytes as f64;
    let mut unit = &UNITS[0];

    for next_unit in &UNITS {
        unit = next_unit;
        if size < 1024.0 {
            break;
        }
        size /= 1024.0;
    }

    format!("{:.2} {}", size, unit)
}
