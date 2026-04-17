use pyo3::prelude::*;
use pyo3::types::{PyDateTime, PyModule, PyTzInfo};
use chrono::{DateTime, Datelike, Timelike, FixedOffset};
use chrono_tz::Tz;
use std::ops::Deref;

//PyTzDateTime allows us to convert between Python datetime with timezone and Rust DateTime<Tz> 
// pyo3's IntoPyObject and FromPyObject traits. 
//This is necessary because pyo3 does not natively support chrono's DateTime<Tz> type, 
//and we need a way to bridge the gap between Python's datetime with tzinfo and Rust's timezone-aware datetime.
#[derive(Debug, Clone, Copy)]
pub struct PyTzDateTime(pub DateTime<Tz>);
// Allows pyo3 getter to work
impl<'py> IntoPyObject<'py> for PyTzDateTime {
    type Target = PyDateTime;
    type Output = Bound<'py, Self::Target>;
    type Error = PyErr;

    fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
        let dt = self.0.naive_local();
        let tz_name = self.0.timezone().name();

        let zoneinfo = PyModule::import(py, "zoneinfo")?;
        let tz_obj = zoneinfo.getattr("ZoneInfo")?.call1((tz_name,))?;
        let tz_bound = tz_obj.cast::<PyTzInfo>()?;

        PyDateTime::new(
            py,
            dt.year(),
            dt.month() as u8,
            dt.day() as u8,
            dt.hour() as u8,
            dt.minute() as u8,
            dt.second() as u8,
            dt.nanosecond() / 1000,
            Some(tz_bound),
        )
    }
}
//Allows pyo3 setter to work
impl<'py, 'a> FromPyObject<'py, 'a> for PyTzDateTime {
    type Error = PyErr;
   fn extract(ob: Borrowed<'py, 'a, PyAny>) -> PyResult<Self> {
        let dt_fixed: DateTime<FixedOffset> = ob.extract()?;
        
        // 2. Access the tzinfo.key attribute
        let tz_info = ob.getattr("tzinfo")?;
        let tz_key: String = tz_info.getattr("key")?.extract()?;

        // 3. Parse back to chrono_tz::Tz
        let tz: Tz = tz_key.parse().map_err(|_| {
            pyo3::exceptions::PyValueError::new_err(format!("Unknown timezone: {}", tz_key))
        })?;

        Ok(PyTzDateTime(dt_fixed.with_timezone(&tz)))
    }
}


impl Deref for PyTzDateTime {
    type Target = DateTime<Tz>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}