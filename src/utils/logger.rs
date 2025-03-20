use flexi_logger::{Logger, LogSpecBuilder, WriteMode};
use log::LevelFilter;
use std::path::PathBuf;
use chrono::prelude::*;

pub fn timestamp() -> String {
    let now: DateTime<Local> = Local::now();
    now.format("%Y-%m-%d %H:%M:%S%.3f").to_string()
}


/// Get the standard logging directory based on the operating system
fn get_log_directory() -> PathBuf {
    if let Some(proj_dirs) = dirs::data_local_dir() {
        // Use the OS-specific local data directory 
        // (e.g., AppData/Local on Windows, 
        // ~/.local/share on Linux, 
        // ~/Library/Application Support on macOS)
        proj_dirs.join("casdesk").join("logs") // Create a "casdesk/logs" subdirectory
    } else {
        // Fallback to the current directory if the standard directory cannot be determined
        PathBuf::from("logs")
    }
}

pub fn init_logger() -> Result<(), Box<dyn std::error::Error>> {
    // Define the log specification (e.g., log level for different modules)
    let mut log_spec_builder = LogSpecBuilder::new();
    log_spec_builder.default(LevelFilter::Info); // Set default log level to Info

    // Get the log directory
    let log_directory = get_log_directory();

    // Initialize the logger
    Logger::with(log_spec_builder.build())
        .log_to_file(
            flexi_logger::FileSpec::default()
                .directory(log_directory) // Use the OS-specific log directory
                .suppress_timestamp(),    // Suppress timestamp in the filename
        )
        .write_mode(WriteMode::BufferAndFlush)        // Buffer logs and flush periodically
        .format(flexi_logger::colored_default_format) // Use human-readable colored format
        .rotate(
            flexi_logger::Criterion::Size(10 * 1024 * 1024), // Rotate when file size exceeds 10 MB
            flexi_logger::Naming::Numbers,                   // Use numbered log files
            flexi_logger::Cleanup::KeepLogFiles(5),          // Keep the last 5 log files
        )
        .start()?;

    Ok(())
}
