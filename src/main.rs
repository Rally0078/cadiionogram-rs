mod read_cadi;
mod siteinfo;
mod pytzdatetime;

use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::Path;
use crate::read_cadi::MDReader;
use chrono::Datelike;
use std::env;

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    
    if args.len() < 4 {
        println!("Usage: {} <input_file> <output_csv> <output_json>", args[0]);
        return Ok(());
    }

    let input_path = Path::new(&args[1]);
    let output_csv_path = Path::new(&args[2]);
    let output_json_path = Path::new(&args[3]);

    if !input_path.exists() {
        eprintln!("Error: {} not found.", input_path.display());
        std::process::exit(1);
    }

    println!("Reading {}...", input_path.display());
    let data = MDReader::read_raw_data(input_path)?;

    // 1. Export Metadata to JSON
    println!("Exporting metadata to {}...", output_json_path.display());
    let json_file = File::create(output_json_path)?;
    serde_json::to_writer_pretty(json_file, &data.metadata)?;

    // 2. Export Data to CSV
    println!("Exporting data to {}...", output_csv_path.display());
    let csv_file = File::create(output_csv_path)?;
    let mut writer = BufWriter::new(csv_file);

    // Write header
    /*write!(writer, "year\tmonth\tday\thour\tminute\tsecond\tfreq (Hz)\theight (km)\tdopplershift")?;
    for i in 1..=data.metadata.noofreceivers {
        write!(writer, "\tsensor{} real\tsensor{} imag", i, i)?;
    }
    writeln!(writer)?;*/

    let n_receivers = data.metadata.noofreceivers as usize;
    let vals_per_bin = n_receivers * 2;

    // Prepare timestamp generation logic
    let obs_dt = *data.metadata.datetime;
    let base_date_str = format!("{:04}\t{:02}\t{:02}", obs_dt.year(), obs_dt.month(), obs_dt.day());
    
    let mut current_record_idx = 0;
    
    // time_partitions is a BTreeMap<String, usize>, so it's sorted by time string
    for (time_str, &cumulative_count) in &data.metadata.time_partitions {   
        let formatted_timestr = format!("{:02}\t{:02}\t{:02}", &time_str[0..2], &time_str[3..5], &time_str[6..8]);
        let timestamp = format!("{}\t{}", base_date_str, formatted_timestr);
        
        while current_record_idx < cumulative_count && current_record_idx < data.height.len() {
            let i = current_record_idx;
            
            let freq_val = data.frequency[i]/1e6;
            let freq_formatted = if freq_val == 0.0 { "0.0".to_string() } else {
                format!("{:.6}", freq_val)
            };

            let height_str = format!("{:.1}", data.height[i]);
            let dop_str = format!("{:.7}", data.dop_shifts[i]);

            write!(
                writer,
                "{}\t{}\t{}\t{}",
                timestamp, freq_formatted, height_str, dop_str
            )?;

            // Write the complex signal components
            let start = i * vals_per_bin;
            for j in 0..vals_per_bin {
                write!(writer, "\t{}", data.complex_signal[start + j])?;
            }
            writeln!(writer)?;
            
            current_record_idx += 1;
        }
    }

    writer.flush()?;
    println!("Successfully exported {} records to {}", data.height.len(), output_csv_path.display());

    Ok(())
}
