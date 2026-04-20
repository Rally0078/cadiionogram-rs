use mdreader_rs::siteinfo::SITE_DICT;
use mdreader_rs::read_cadi::MDReader;
use chrono::{NaiveDate, NaiveDateTime, NaiveTime};
use chrono_tz::{Asia::Kolkata, UTC, Tz};
use tempfile::NamedTempFile;
use std::io::Write;

#[test]
fn test_siteinfo() {
    let sitenames = vec!["TIR", "TIR", "ALD", "TFR"];
    let obs_datetimes = vec![
        NaiveDateTime::new(NaiveDate::from_ymd_opt(2019, 10, 5).unwrap(), NaiveTime::from_hms_opt(0, 0, 0).unwrap()),
        NaiveDateTime::new(NaiveDate::from_ymd_opt(2024, 2, 23).unwrap(), NaiveTime::from_hms_opt(0, 0, 0).unwrap()),
        NaiveDateTime::new(NaiveDate::from_ymd_opt(2018, 4, 21).unwrap(), NaiveTime::from_hms_opt(0, 0, 0).unwrap()),
        NaiveDateTime::new(NaiveDate::from_ymd_opt(2026, 7, 15).unwrap(), NaiveTime::from_hms_opt(0, 0, 0).unwrap()),
    ];
    let expected_zones: Vec<Tz> = vec![
        Kolkata.into(),
        UTC.into(),
        Kolkata.into(),
        Kolkata.into(),
    ];

    for (site, obs_dt, expected_zone) in itertools::izip!(sitenames, obs_datetimes, expected_zones) {
        let site_info = SITE_DICT.get(site).expect(&format!("Site {} not found", site));
        assert_eq!(site_info.get_tzinfo(obs_dt), expected_zone);
    }
}

fn write_mock_header(file: &mut NamedTempFile, site: &str, datetime: &str) {
    file.write_all(site.as_bytes()).unwrap(); // 3 bytes
    file.write_all(datetime.as_bytes()).unwrap(); // 22 bytes
    file.write_all(b"H").unwrap(); // 1 byte
    file.write_all(&0u16.to_le_bytes()).unwrap(); // nfreqs
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
    file.write_all(&4u8.to_le_bytes()).unwrap(); // noofreceivers
    file.write_all(b"abcxyzdef32").unwrap(); // Spares (11 bytes)
}

#[test]
fn test_raw_md4_tir_lt() {
    let mut file = NamedTempFile::new().unwrap();
    write_mock_header(&mut file, "TIR", " Jan 20 12:34:56 2010\n");
    
    let data = MDReader::read_raw_data(file.path()).unwrap();
    // In metadata, it is stored in PyTzDateTime
    // We check the offset/name or try to match the timezone
    let tz_str = data.metadata.datetime.0.timezone().to_string();
    assert_eq!(tz_str, "Asia/Kolkata");
}

#[test]
fn test_raw_md4_tir_ut() {
    let mut file = NamedTempFile::new().unwrap();
    write_mock_header(&mut file, "TIR", " Mar 20 12:34:56 2025\n");
    
    let data = MDReader::read_raw_data(file.path()).unwrap();
    let tz_str = data.metadata.datetime.0.timezone().to_string();
    assert_eq!(tz_str, "UTC");
}
