use mdreader_rs::read_cadi::MDReader;
use tempfile::NamedTempFile;
use std::io::Write;

fn write_mock_header(file: &mut NamedTempFile, site: &str, datetime: &str, nfreqs: u16, noofreceivers: u8) {
    file.write_all(site.as_bytes()).unwrap(); // 3 bytes
    file.write_all(datetime.as_bytes()).unwrap(); // 22 bytes
    file.write_all(b"H").unwrap(); // 1 byte
    file.write_all(&nfreqs.to_le_bytes()).unwrap(); // 2 bytes
    file.write_all(&2u8.to_le_bytes()).unwrap(); // ndops
    file.write_all(&90u16.to_le_bytes()).unwrap(); // minheight
    file.write_all(&1024u16.to_le_bytes()).unwrap(); // maxheight
    file.write_all(&8u8.to_le_bytes()).unwrap(); // pps
    file.write_all(&3u8.to_le_bytes()).unwrap(); // npulses_avgd
    file.write_all(&400u16.to_le_bytes()).unwrap(); // base_thr100
    file.write_all(&135u16.to_le_bytes()).unwrap(); // noise_thr100
    file.write_all(&1u8.to_le_bytes()).unwrap(); // min_dops_save
    file.write_all(&60u16.to_le_bytes()).unwrap(); // dtime
    file.write_all(b"2").unwrap(); // gain_control
    file.write_all(b"F").unwrap(); // sig_process
    file.write_all(&noofreceivers.to_le_bytes()).unwrap(); // noofreceivers
    file.write_all(b"abcxyzdef32").unwrap(); // Spares (11 bytes)
}

#[test]
fn test_read_raw_file_tir_lt() {
    let mut file = NamedTempFile::new().unwrap();
    let nfreqs = 4u16;
    let noofreceivers = 4u8;
    
    write_mock_header(&mut file, "TIR", " Jan 20 12:34:56 2010\n", nfreqs, noofreceivers);
    
    let freq_list_mock = [3e6f32, 6e6f32, 9e6f32, 12e6f32];
    for &freq in &freq_list_mock {
        file.write_all(&freq.to_le_bytes()).unwrap();
    }

    // Record 1
    file.write_all(&10u8.to_le_bytes()).unwrap(); // time_min
    file.write_all(&30u8.to_le_bytes()).unwrap(); // time_sec
    file.write_all(&226u8.to_le_bytes()).unwrap(); // gain_flag
    
    for _freqx in 0..nfreqs {
        file.write_all(&32u8.to_le_bytes()).unwrap(); // noise_flag
        file.write_all(&384u16.to_le_bytes()).unwrap(); // noise_power10
        
        // Height 1
        file.write_all(&100u8.to_le_bytes()).unwrap(); // hflag (height = 100 * 3 = 300)
        file.write_all(&1u8.to_le_bytes()).unwrap(); // ndops_oneh
        file.write_all(&5u8.to_le_bytes()).unwrap(); // dop_flag
        for _ in 0..noofreceivers {
            file.write_all(&[10u8, 20u8]).unwrap(); // Re, Im
        }
        
        // Stop heights for this freq
        file.write_all(&226u8.to_le_bytes()).unwrap();
    }
    
    // End of file
    file.write_all(&255u8.to_le_bytes()).unwrap();

    let data = MDReader::read_raw_data(file.path()).unwrap();
    
    assert_eq!(data.metadata.site, "TIR");
    assert_eq!(data.metadata.nfreqs, 4);
    assert_eq!(data.height.len(), 4); // 1 record * 4 freqs * 1 height
    assert_eq!(data.height[0], 300.0);
    assert_eq!(data.frequency[0], 3e6);
    assert_eq!(data.metadata.time_partitions.len(), 1);
    assert!(data.metadata.time_partitions.contains_key("12:10:30"));
}

#[test]
fn test_read_raw_file_incomplete_header() {
    let mut file = NamedTempFile::new().unwrap();
    file.write_all(b"TIR").unwrap();
    file.write_all(b" Jan 20 12:34:56 2010\n").unwrap();
    // File ends here
    
    let data = MDReader::read_raw_data(file.path()).unwrap();
    assert!(data.metadata.incomplete_header);
    assert!(data.height.is_empty());
}

#[test]
fn test_read_raw_file_incomplete_data() {
    let mut file = NamedTempFile::new().unwrap();
    let nfreqs = 4u16;
    let noofreceivers = 4u8;
    
    write_mock_header(&mut file, "TIR", " Jan 20 12:34:56 2010\n", nfreqs, noofreceivers);
    
    let freq_list_mock = [3e6f32, 6e6f32, 9e6f32, 12e6f32];
    for &freq in &freq_list_mock {
        file.write_all(&freq.to_le_bytes()).unwrap();
    }

    // Record 1 - complete
    file.write_all(&10u8.to_le_bytes()).unwrap();
    file.write_all(&30u8.to_le_bytes()).unwrap();
    file.write_all(&226u8.to_le_bytes()).unwrap();
    for _ in 0..nfreqs {
        file.write_all(&[32u8, 128u8, 1u8, 100u8, 1u8, 5u8]).unwrap(); // noise_flag, noise_power (low), noise_power (high), hflag, ndops_oneh, dop_flag
        for _ in 0..noofreceivers {
            file.write_all(&[10u8, 20u8]).unwrap();
        }
        file.write_all(&226u8.to_le_bytes()).unwrap();
    }

    // Record 2 - incomplete
    file.write_all(&11u8.to_le_bytes()).unwrap();
    // File ends prematurely
    
    let data = MDReader::read_raw_data(file.path()).unwrap();
    assert!(data.metadata.incomplete_data);
    assert_eq!(data.metadata.time_partitions.len(), 1); // Only the first record should be kept
    assert_eq!(data.height.len(), 4);
}
