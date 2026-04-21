use chrono::{NaiveDate, NaiveDateTime, NaiveTime};
use chrono_tz::{Asia::Kolkata, UTC, Tz};
use std::collections::HashMap;
use once_cell::sync::Lazy;

#[derive(Debug, Clone, Copy)]
#[allow(dead_code)]
pub struct SiteInfo {
    pub fh: f32,
    pub dip: f32,
    pub site: &'static str,
    pub short_site: &'static str,
}

impl SiteInfo {
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
        let tz = self.get_tzinfo(dtime);
        match tz {
            Tz::UTC => "UT",
            _ => "LT",
        }
    }
}

pub static SITE_DICT: Lazy<HashMap<&'static str, SiteInfo>> = Lazy::new(|| {
    let mut m = HashMap::new();
    let ald = SiteInfo { fh: 1.119, dip: 10.2, site: "ALD", short_site: "al" };
    let tir = SiteInfo { fh: 0.951, dip: 0.5, site: "TIR", short_site: "ti" };
    let hyd = SiteInfo { fh: 1.007, dip: 6.5, site: "TFR", short_site: "tf" };
    let moc = SiteInfo { fh: 0.0, dip: 0.0, site: "MOC", short_site: "ut" };

    m.insert("TIR", tir);
    m.insert("KSKGRL-IIGM PRAYAGRAJ", ald);
    m.insert("ALD", ald);
    m.insert("TFR", hyd);
    m.insert("MOC", moc);
    m
});
