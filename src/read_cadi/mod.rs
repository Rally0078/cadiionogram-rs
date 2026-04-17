//Library crate containing the CADI binary file reader
use crate::siteinfo::SITE_DICT;
use chrono::{NaiveDate, NaiveDateTime, NaiveTime, Timelike, TimeZone};
use std::fs::File;
use std::io::{BufReader, Read, Seek, SeekFrom};
use std::path::Path;
pub mod cadi_dataclass;
pub use cadi_dataclass::{Metadata, CadiData};
use crate::pytzdatetime::PyTzDateTime;

pub struct MDReader;

impl MDReader {
    pub fn read_raw_data(filename: &Path) -> std::io::Result<CadiData> {
        let mut context = match ReaderContext::new(filename) {
            Ok(ctx) => ctx,
            Err(e) => return Err(e),
        };

        context.read()
    }

    fn convert_bins_to_vals(
        dopbin_x_freqx: &[u16],
        dopbin_x_hflag: &[u16],
        dopbin_x_dop_flag: &[u8],
        dopbin_iq: &[Vec<u8>],
        noofreceivers: u8,
        freqs: &[f32],
        ndops: u8,
        npulses_avgd: u8,
        pps: u8,
    ) -> (Vec<f32>, Vec<f32>, Vec<f32>, Vec<i16>) {
        let n = dopbin_iq.len();
        if n == 0 {
            return (Vec::new(), Vec::new(), Vec::new(), Vec::new());
        }

        let dopsn2 = if ndops > 0 && npulses_avgd > 0 && pps > 0 {
            1.0 / (ndops as f64 * npulses_avgd as f64 / pps as f64)
        } else {
            0.0
        };
        let ndops_f64 = ndops as f64 / 2.0;

        let mut height = Vec::with_capacity(n);
        let mut frequency = Vec::with_capacity(n);
        let mut dop_shifts = Vec::with_capacity(n);
        let mut complex_signal = Vec::with_capacity(n * noofreceivers as usize * 2);

        for i in 0..n {
            height.push(dopbin_x_hflag[i] as f32 * 3.0);
            frequency.push(freqs[dopbin_x_freqx[i] as usize]);
            dop_shifts.push(((dopbin_x_dop_flag[i] as f64 - ndops_f64) * dopsn2) as f32);
            complex_signal.extend(dopbin_iq[i].iter().map(|&v| v as i8 as i16));
        }

        (height, frequency, dop_shifts, complex_signal)
    }
}

struct ReaderContext {
    reader: BufReader<File>,
    eof: u64,
    metadata: Metadata,
}

impl ReaderContext {
    fn new(filename:  &Path) -> std::io::Result<Self> {
        let file = File::open(filename)?;
        let mut reader = BufReader::new(file);

        let eof = reader.seek(SeekFrom::End(0))?;
        reader.seek(SeekFrom::Start(0))?;

        let extension = filename
            .extension()
            .and_then(|e| e.to_str())
            .unwrap_or("unknown")
            .to_string();

        let metadata = Metadata {
            source: filename.file_name().unwrap().to_string_lossy().to_string(),
            extension,
            dheight: 3.0,
            ..Default::default()
        };

        Ok(Self {
            reader,
            eof,
            metadata,
        })
    }

    fn read(&mut self) -> std::io::Result<CadiData> {
        if let Err(_) = self.read_header() {
            return Ok(CadiData::empty(self.metadata.clone()));
        }

        let mut freqs = Vec::with_capacity(self.metadata.nfreqs as usize);
        for _ in 0..self.metadata.nfreqs {
            match self.read_f32() {
                Ok(f) => freqs.push(f),
                Err(_) => {
                    self.metadata.incomplete_header = true;
                    return Ok(CadiData {
                        freqs,
                        ..CadiData::empty(self.metadata.clone())
                    });
                }
            }
        }

        let mut observations = ObservationBuffer::new();
        if let Err(_) = self.read_records(&mut observations) {
            // Truncate partial data for the incomplete record
            if let Some(&final_idx) = self.metadata.time_partitions.values().last() {
                observations.truncate(final_idx);
            } else {
                // If no records were completed, return empty data
                return Ok(CadiData {
                    freqs,
                    ..CadiData::empty(self.metadata.clone())
                });
            }
        }

        let (height, frequency, dop_shifts, complex_signal) = MDReader::convert_bins_to_vals(
            &observations.dopbin_x_freqx,
            &observations.dopbin_x_hflag,
            &observations.dopbin_x_dop_flag,
            &observations.dopbin_iq,
            self.metadata.noofreceivers,
            &freqs,
            self.metadata.ndops,
            self.metadata.npulses_avgd,
            self.metadata.pps,
        );

        Ok(CadiData {
            file_list: observations.file_list,
            metadata: self.metadata.clone(),
            height,
            frequency,
            freqs,
            dop_shifts,
            complex_signal,
        })
    }

    fn read_header(&mut self) -> Result<(), ()> {
        self.metadata.site = self.read_string(3).map_err(|_| self.mark_header_incomplete())?.trim().to_string();
        let ascii_datetime = self.read_string(22).map_err(|_| self.mark_header_incomplete())?;
        
        // Parse ascii_datetime: " Jan 01 00:00:00 2020 "
        let month_str = ascii_datetime.get(1..4).ok_or(())?;
        let day = ascii_datetime.get(5..7).and_then(|s| s.trim().parse::<u32>().ok()).ok_or(())?;
        let hour = ascii_datetime.get(8..10).and_then(|s| s.parse::<u32>().ok()).ok_or(())?;
        let minute = ascii_datetime.get(11..13).and_then(|s| s.parse::<u32>().ok()).ok_or(())?;
        let sec = ascii_datetime.get(14..16).and_then(|s| s.parse::<u32>().ok()).ok_or(())?;
        let year = ascii_datetime.get(17..21).and_then(|s| s.parse::<i32>().ok()).ok_or(())?;

        let month = match month_str {
            "Jan" => 1, "Feb" => 2, "Mar" => 3, "Apr" => 4,
            "May" => 5, "Jun" => 6, "Jul" => 7, "Aug" => 8,
            "Sep" => 9, "Oct" => 10, "Nov" => 11, "Dec" => 12,
            _ => 1,
        };

        let naive_date = NaiveDate::from_ymd_opt(year, month, day).ok_or(())?;
        let naive_time = NaiveTime::from_hms_opt(hour, minute, sec).ok_or(())?;
        let naive_datetime = NaiveDateTime::new(naive_date, naive_time);
        // Use siteinfo for timezone resolution for each site
        let tz = SITE_DICT.get(self.metadata.site.as_str())
            .map(|info| info.get_tzinfo(naive_datetime))
            .unwrap_or(chrono_tz::UTC);

        self.metadata.datetime = PyTzDateTime(tz.from_local_datetime(&naive_datetime).unwrap());

        self.metadata.filetype = self.read_string(1).map_err(|_| self.mark_header_incomplete())?;
        self.metadata.nfreqs = self.read_u16().map_err(|_| self.mark_header_incomplete())?;
        self.metadata.ndops = self.read_u8().map_err(|_| self.mark_header_incomplete())?;
        self.metadata.minheight = self.read_u16().map_err(|_| self.mark_header_incomplete())?;
        self.metadata.maxheight = self.read_u16().map_err(|_| self.mark_header_incomplete())?;
        self.metadata.pps = self.read_u8().map_err(|_| self.mark_header_incomplete())?;
        self.metadata.npulses_avgd = self.read_u8().map_err(|_| self.mark_header_incomplete())?;
        
        let _base_thr100 = self.read_u16().map_err(|_| self.mark_header_incomplete())?;
        let _noise_thr100 = self.read_u16().map_err(|_| self.mark_header_incomplete())?;
        let _min_dop_forsave = self.read_u8().map_err(|_| self.mark_header_incomplete())?;
        
        self.metadata.dtime = self.read_u16().map_err(|_| self.mark_header_incomplete())?;
        let _gain_control = self.read_u8().map_err(|_| self.mark_header_incomplete())?;
        let _sig_process = self.read_u8().map_err(|_| self.mark_header_incomplete())?;
        self.metadata.noofreceivers = self.read_u8().map_err(|_| self.mark_header_incomplete())?;
        
        let mut spares = [0u8; 11];
        self.reader.read_exact(&mut spares).map_err(|_| self.mark_header_incomplete())?;

        self.metadata.nheights = (self.metadata.maxheight as f32 / self.metadata.dheight + 1.0) as u32;

        Ok(())
    }

    fn read_records(&mut self, obs: &mut ObservationBuffer) -> Result<(), ()> {
        let mut time_min = self.read_u8().map_err(|_| ())?;
        let hour = self.metadata.datetime.hour();

        while self.pos() < self.eof && time_min != 255 && time_min < 60 {
            let time_sec = self.read_u8().map_err(|_| self.mark_data_incomplete())?;
            let _gain_flag = self.read_u8().map_err(|_| self.mark_data_incomplete())?;

            let time_partition_key = format!("{:02}:{:02}:{:02}", hour, time_min, time_sec);

            for freqx in 0..self.metadata.nfreqs {
                let _noise_flag = self.read_u8().map_err(|_| self.mark_data_incomplete())?;
                let _noise_power10 = self.read_u16().map_err(|_| self.mark_data_incomplete())?;
                let mut flag = self.read_u8().map_err(|_| self.mark_data_incomplete())?;

                while flag < 224 {
                    let mut ndops_oneh = self.read_u8().map_err(|_| self.mark_data_incomplete())?;
                    let mut hflag = flag as u16;

                    if ndops_oneh >= 128 {
                        ndops_oneh -= 128;
                        hflag += 200;
                    }

                    for _ in 0..ndops_oneh {
                        let mut dop_flag = self.read_u8().map_err(|_| self.mark_data_incomplete())?;
                        let mut iq_data = Vec::with_capacity(self.metadata.noofreceivers as usize * 2);

                        for _ in 0..self.metadata.noofreceivers {
                            iq_data.push(self.read_u8().map_err(|_| self.mark_data_incomplete())?);
                            iq_data.push(self.read_u8().map_err(|_| self.mark_data_incomplete())?);
                        }

                        obs.dopbin_iq.push(iq_data);
                        obs.dopbin_x_freqx.push(freqx);
                        obs.dopbin_x_hflag.push(hflag);

                        let ndops_half = self.metadata.ndops / 2;
                        if dop_flag < ndops_half {
                            dop_flag += ndops_half;
                        } else {
                            dop_flag -= ndops_half;
                        }
                        obs.dopbin_x_dop_flag.push(dop_flag);
                    }
                    flag = self.read_u8().map_err(|_| self.mark_data_incomplete())?;
                }
                time_min = flag;
            }

            self.metadata.time_partitions.insert(time_partition_key, obs.dopbin_iq.len());
            obs.file_list.push(self.metadata.source.clone());

            if self.pos() < self.eof {
                match self.read_u8() {
                    Ok(v) => time_min = v,
                    Err(_) => break,
                }
            }
        }
        Ok(())
    }

    fn pos(&mut self) -> u64 {
        self.reader.stream_position().unwrap_or(self.eof)
    }

    fn read_u8(&mut self) -> std::io::Result<u8> {
        let mut b = [0u8; 1];
        self.reader.read_exact(&mut b)?;
        Ok(b[0])
    }

    fn read_u16(&mut self) -> std::io::Result<u16> {
        let mut b = [0u8; 2];
        self.reader.read_exact(&mut b)?;
        Ok(u16::from_le_bytes(b))
    }

    fn read_f32(&mut self) -> std::io::Result<f32> {
        let mut b = [0u8; 4];
        self.reader.read_exact(&mut b)?;
        Ok(f32::from_le_bytes(b))
    }

    fn read_string(&mut self, len: usize) -> std::io::Result<String> {
        let mut b = vec![0u8; len];
        self.reader.read_exact(&mut b)?;
        Ok(String::from_utf8_lossy(&b).to_string())
    }

    fn mark_header_incomplete(&mut self) {
        self.metadata.incomplete_header = true;
        self.metadata.incomplete_data = true;
    }

    fn mark_data_incomplete(&mut self) {
        self.metadata.incomplete_data = true;
    }
}

struct ObservationBuffer {
    dopbin_x_freqx: Vec<u16>,
    dopbin_x_hflag: Vec<u16>,
    dopbin_x_dop_flag: Vec<u8>,
    dopbin_iq: Vec<Vec<u8>>,
    file_list: Vec<String>,
}

impl ObservationBuffer {
    fn new() -> Self {
        Self {
            dopbin_x_freqx: Vec::new(),
            dopbin_x_hflag: Vec::new(),
            dopbin_x_dop_flag: Vec::new(),
            dopbin_iq: Vec::new(),
            file_list: Vec::new(),
        }
    }

    fn truncate(&mut self, len: usize) {
        self.dopbin_x_freqx.truncate(len);
        self.dopbin_x_hflag.truncate(len);
        self.dopbin_x_dop_flag.truncate(len);
        self.dopbin_iq.truncate(len);
        // file_list length should already match number of entries in time_partitions
    }
}
