use chrono::{NaiveDate, NaiveDateTime, NaiveTime};
use chrono_tz::{Asia::Kolkata, Tz, UTC};
use once_cell::sync::OnceCell;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::env;
use std::fs;
use std::path::PathBuf;

const DEFAULT_SITES_JSON: &str = r#"{
    "TIR": {
        "FH": 0.951,
        "dip": 0.5,
        "site": "TIR",
        "short_site": "ti",
        "ph_corr": [8.8906, -29.5086],
        "polarity": [1.0, -1.0, 1.0, -1.0],
        "site_separation": [30.1, 30.1]
    },
    "KSKGRL-IIGM PRAYAGRAJ": {
        "FH": 1.119,
        "dip": 10.2,
        "site": "ALD",
        "short_site": "al",
        "ph_corr": [0.0, 0.0],
        "polarity": [1.0, -1.0, 1.0, -1.0],
        "site_separation": [20.1, 20.1]
    },
    "ALD": {
        "FH": 1.119,
        "dip": 10.2,
        "site": "ALD",
        "short_site": "al",
        "ph_corr": [0.0, 0.0],
        "polarity": [1.0, -1.0, 1.0, -1.0],
        "site_separation": [20.1, 20.1]
    },
    "TFR": {
        "FH": 1.007,
        "dip": 6.5,
        "site": "TFR",
        "short_site": "tf",
        "ph_corr": [0.0, 0.0],
        "polarity": [1.0, -1.0, 1.0, -1.0],
        "site_separation": [30.1, 30.1]
    },
    "MOC": {
        "FH": 0.0,
        "dip": 0.0,
        "site": "MOC",
        "short_site": "ut",
        "ph_corr": [0.0, 0.0],
        "polarity": [1.0, -1.0, 1.0, -1.0],
        "site_separation": [15.0, 15.0]
    }
}"#;


#[derive(Debug, Clone, Serialize, Deserialize)]
struct SiteInfoRaw {
    #[serde(rename = "FH")]
    fh: f64,
    dip: f64,
    site: String,
    short_site: String,
    ph_corr: Vec<f64>,
    polarity: Vec<f64>,
    site_separation: Vec<f64>,
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct SiteInfo {
    pub fh: f64,
    pub dip: f64,
    pub site: String,
    pub short_site: String,
    pub ph_corr: Vec<f64>,
    pub polarity: Vec<f64>,
    pub site_separation: Vec<f64>,
}

impl From<SiteInfoRaw> for SiteInfo {
    fn from(raw: SiteInfoRaw) -> Self {
        SiteInfo {
            fh: raw.fh,
            dip: raw.dip,
            site: raw.site,
            short_site: raw.short_site,
            ph_corr: raw.ph_corr,
            polarity: raw.polarity,
            site_separation: raw.site_separation,
        }
    }
}


pub fn get_sites_json_path() -> PathBuf {
    #[cfg(target_os = "windows")]
    {
        let is_frozen = env::var("CARGO_MANIFEST_DIR").is_err();
        if is_frozen {
            let exe = env::current_exe().expect("Cannot determine executable path");
            exe.parent()
                .expect("Executable has no parent directory")
                .join("sites.json")
        } else {
            PathBuf::from("./sites.json")
        }
    }
    #[cfg(not(target_os = "windows"))]
    {
        let home = env::var_os("HOME")
            .map(PathBuf::from)
            .unwrap_or_else(|| PathBuf::from("."));
        home.join(".config").join("egrliono").join("sites.json")
    }
}

type SiteMap = HashMap<String, SiteInfo>;

static SITE_CACHE: OnceCell<SiteMap> = OnceCell::new();

pub fn load_sites_file() -> &'static SiteMap {
    SITE_CACHE.get_or_init(|| {
        let path = get_sites_json_path();

        if !path.exists() {
            if let Some(parent) = path.parent() {
                fs::create_dir_all(parent)
                    .expect("Failed to create config directory for sites.json");
            }
            fs::write(&path, DEFAULT_SITES_JSON)
                .expect("Failed to write default sites.json");
        }

        let contents =
            fs::read_to_string(&path).expect("Failed to read sites.json");
        let raw_map: HashMap<String, SiteInfoRaw> =
            serde_json::from_str(&contents).expect("Failed to parse sites.json");

        raw_map
            .into_iter()
            .map(|(k, v)| (k, SiteInfo::from(v)))
            .collect()
    })
}


impl SiteInfo {
    pub fn from_file(site_name: &str) -> Result<SiteInfo, String> {
        load_sites_file()
            .get(site_name)
            .cloned()
            .ok_or_else(|| format!("Site '{}' not found in sites.json", site_name))
    }

    pub fn get_from_file(site_name: &str) -> Option<SiteInfo> {
        Self::from_file(site_name).ok()
    }

    pub fn get_tzinfo(&self, dtime: NaiveDateTime) -> Tz {
        if self.site == "TIR" {
            let tir_threshold = NaiveDateTime::new(
                NaiveDate::from_ymd_opt(2020, 1, 1).unwrap(),
                NaiveTime::from_hms_opt(0, 0, 0).unwrap(),
            );
            if dtime < tir_threshold {
                Kolkata.into()
            } else {
                UTC.into()
            }
        } else {
            Kolkata.into()
        }
    }

    #[allow(dead_code)]
    pub fn get_tzstr(&self, dtime: NaiveDateTime) -> &'static str {
        match self.get_tzinfo(dtime) {
            Tz::UTC => "UT",
            _ => "LT",
        }
    }
}
