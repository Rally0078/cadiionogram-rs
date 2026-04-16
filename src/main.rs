mod read_cadi;
mod siteinfo;

use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::Path;
use crate::read_cadi::MDReader;

fn main() -> std::io::Result<()> {
    let input_path = Path::new("input.md4");
    let output_path = Path::new("output.csv");

    if !input_path.exists() {
        eprintln!("Error: {} not found.", input_path.display());
        return Ok(());
    }

    println!("Reading {}...", input_path.display());
    let data = MDReader::read_raw_data(input_path)?;

    println!("Exporting to {}...", output_path.display());
    let file = File::create(output_path)?;
    let mut writer = BufWriter::new(file);

    // Write header
    // Row format: freq (Hz),height (km),dopplershift,sensor1 real,sensor1 imag, ...
    write!(writer, "freq (Hz),height (km),dopplershift")?;
    for i in 1..=data.metadata.noofreceivers {
        write!(writer, ",sensor{} real,sensor{} imag", i, i)?;
    }
    writeln!(writer)?;

    let n_recs = data.metadata.noofreceivers as usize;
    let vals_per_bin = n_recs * 2;

    for i in 0..data.height.len() {
        // Write the core data columns: freq, height, dop_shift
        // Match pandas float formatting: e+06 scientific notation and ensuring .0 for integers
        let freq_str = {
            let s = format!("{:e}", data.frequency[i]);
            let parts: Vec<&str> = s.split('e').collect();
            let exp: i32 = parts[1].parse().unwrap_or(0);
            format!("{}e{:+03}", parts[0], exp)
        };
        let height_str = {
            let s = format!("{}", data.height[i]);
            if !s.contains('.') && !s.contains('e') {
                format!("{}.0", s)
            } else {
                s
            }
        };
        let dop_str = {
            let s = format!("{}", data.dop_shifts[i]);
            if !s.contains('.') && !s.contains('e') {
                format!("{}.0", s)
            } else {
                s
            }
        };

        write!(
            writer,
            "{},{},{}",
            freq_str, height_str, dop_str
        )?;

        // Write the complex signal components for all receivers
        let start = i * vals_per_bin;
        for j in 0..vals_per_bin {
            write!(writer, ",{}", data.complex_signal[start + j])?;
        }
        writeln!(writer)?;
    }

    writer.flush()?;
    println!("Successfully exported {} records to {}", data.height.len(), output_path.display());

    Ok(())
}
