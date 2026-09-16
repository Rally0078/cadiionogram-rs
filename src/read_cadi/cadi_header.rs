use crate::pytzdatetime::PyTzDateTime;
use chrono::DateTime;
use chrono_tz::UTC;
use pyo3::IntoPyObjectExt;
use pyo3::prelude::*;
use serde::Serialize;

///
/// Contains the CADI header data from a given `mdX(X=1,2,3,4)` file.
///
#[pyclass(dict, from_py_object)]
#[derive(Debug, Clone, Serialize)]
pub struct CADIheader {
    #[pyo3(get, set)]
    pub site: String,
    #[pyo3(get, set)]
    pub datetime: PyTzDateTime,
    #[pyo3(get, set)]
    pub source: String,
    #[pyo3(get, set)]
    pub filetype: String,
    #[pyo3(get, set)]
    pub ndops: u8,
    #[pyo3(get, set)]
    pub nfreqs: u16,
    #[pyo3(get, set)]
    pub nheights: u32,
    #[pyo3(get, set)]
    pub minheight: u16,
    #[pyo3(get, set)]
    pub maxheight: u16,
    #[pyo3(get, set)]
    pub dheight: f32,
    #[pyo3(get, set)]
    pub pps: u8,
    #[pyo3(get, set)]
    pub npulses_avgd: u8,
    #[pyo3(get, set)]
    pub dtime: u16,
    #[pyo3(get, set)]
    pub base_thr100: u16,
    #[pyo3(get, set)]
    pub noise_thr100: u16,
    #[pyo3(get, set)]
    pub min_dop_forsave: u8,
    #[pyo3(get, set)]
    pub gain_control: String,
    #[pyo3(get, set)]
    pub sig_process: String,
    #[pyo3(get, set)]
    pub spares: Vec<u8>,
    #[pyo3(get, set)]
    pub extension: String,
    #[pyo3(get, set)]
    pub noofreceivers: u8,
    #[pyo3(get, set, name = "incompletedata")]
    pub incomplete_data: bool,
    #[pyo3(get, set, name = "incompleteheader")]
    pub incomplete_header: bool,
}

#[pymethods]
impl CADIheader {
    fn __getitem__(&self, key: &str, py: Python<'_>) -> PyResult<Py<PyAny>> {
        match key {
            "site" => self.site.clone().into_py_any(py),
            "datetime" => self.datetime.into_py_any(py),
            //self.datetime.into_py_any(py),
            "source" => self.source.clone().into_py_any(py),
            "filetype" => self.filetype.clone().into_py_any(py),
            "ndops" => self.ndops.into_py_any(py),
            "nfreqs" => self.nfreqs.into_py_any(py),
            "nheights" => self.nheights.into_py_any(py),
            "minheight" => self.minheight.into_py_any(py),
            "maxheight" => self.maxheight.into_py_any(py),
            "dheight" => self.dheight.into_py_any(py),
            "pps" => self.pps.into_py_any(py),
            "npulses_avgd" => self.npulses_avgd.into_py_any(py),
            "dtime" => self.dtime.into_py_any(py),
            "base_thr100" => self.base_thr100.into_py_any(py),
            "noise_thr100" => self.noise_thr100.into_py_any(py),
            "min_dop_forsave" => self.min_dop_forsave.into_py_any(py),
            "gain_control" => self.gain_control.clone().into_py_any(py),
            "sig_process" => self.sig_process.clone().into_py_any(py),
            "spares" => self.spares.clone().into_py_any(py),
            "extension" => self.extension.clone().into_py_any(py),
            "noofreceivers" => self.noofreceivers.into_py_any(py),
            "incompletedata" => self.incomplete_data.into_py_any(py),
            "incompleteheader" => self.incomplete_header.into_py_any(py),
            _ => Err(pyo3::exceptions::PyKeyError::new_err(key.to_string())),
        }
    }

    fn __setitem__(&mut self, key: &str, value: Bound<'_, PyAny>) -> PyResult<()> {
        match key {
            "site" => self.site = value.extract()?,
            "datetime" => self.datetime = value.extract()?,
            "source" => self.source = value.extract()?,
            "filetype" => self.filetype = value.extract()?,
            "ndops" => self.ndops = value.extract()?,
            "nfreqs" => self.nfreqs = value.extract()?,
            "nheights" => self.nheights = value.extract()?,
            "minheight" => self.minheight = value.extract()?,
            "maxheight" => self.maxheight = value.extract()?,
            "dheight" => self.dheight = value.extract()?,
            "pps" => self.pps = value.extract()?,
            "npulses_avgd" => self.npulses_avgd = value.extract()?,
            "dtime" => self.dtime = value.extract()?,
            "base_thr100" => self.base_thr100 = value.extract()?,
            "noise_thr100" => self.noise_thr100 = value.extract()?,
            "min_dop_forsave" => self.min_dop_forsave = value.extract()?,
            "gain_control" => self.gain_control = value.extract()?,
            "sig_process" => self.sig_process = value.extract()?,
            "spares" => self.spares = value.extract()?,
            "extension" => self.extension = value.extract()?,
            "noofreceivers" => self.noofreceivers = value.extract()?,
            "incompletedata" => self.incomplete_data = value.extract()?,
            "incompleteheader" => self.incomplete_header = value.extract()?,
            _ => return Err(pyo3::exceptions::PyKeyError::new_err(key.to_string())),
        }
        Ok(())
    }

    fn keys(&self) -> Vec<&'static str> {
        vec![
            "site",
            "datetime",
            "source",
            "filetype",
            "ndops",
            "nfreqs",
            "nheights",
            "minheight",
            "maxheight",
            "dheight",
            "pps",
            "npulses_avgd",
            "dtime",
            "base_thr100",
            "noise_thr100",
            "min_dop_forsave",
            "gain_control",
            "sig_process",
            "spares",
            "extension",
            "noofreceivers",
            "incompletedata",
            "incompleteheader",
        ]
    }

    fn items(&self, py: Python<'_>) -> PyResult<Vec<(&'static str, Py<PyAny>)>> {
        Ok(vec![
            ("site", self.site.clone().into_py_any(py)?),
            ("datetime", self.datetime.into_py_any(py)?),
            ("source", self.source.clone().into_py_any(py)?),
            ("filetype", self.filetype.clone().into_py_any(py)?),
            ("ndops", self.ndops.into_py_any(py)?),
            ("nfreqs", self.nfreqs.into_py_any(py)?),
            ("nheights", self.nheights.into_py_any(py)?),
            ("minheight", self.minheight.into_py_any(py)?),
            ("maxheight", self.maxheight.into_py_any(py)?),
            ("dheight", self.dheight.into_py_any(py)?),
            ("pps", self.pps.into_py_any(py)?),
            ("npulses_avgd", self.npulses_avgd.into_py_any(py)?),
            ("dtime", self.dtime.into_py_any(py)?),
            ("base_thr100", self.base_thr100.into_py_any(py)?),
            ("noise_thr100", self.noise_thr100.into_py_any(py)?),
            ("min_dop_forsave", self.min_dop_forsave.into_py_any(py)?),
            ("gain_control", self.gain_control.clone().into_py_any(py)?),
            ("sig_process", self.sig_process.clone().into_py_any(py)?),
            ("spares", self.spares.clone().into_py_any(py)?),
            ("extension", self.extension.clone().into_py_any(py)?),
            ("noofreceivers", self.noofreceivers.into_py_any(py)?),
            ("incompletedata", self.incomplete_data.into_py_any(py)?),
            ("incompleteheader", self.incomplete_header.into_py_any(py)?),
        ])
    }

    #[new]
    pub fn new(
        site: String,
        datetime: PyTzDateTime,
        source: String,
        filetype: String,
        ndops: u8,
        nfreqs: u16,
        nheights: u32,
        minheight: u16,
        maxheight: u16,
        dheight: f32,
        pps: u8,
        npulses_avgd: u8,
        dtime: u16,
        base_thr100: u16,
        noise_thr100: u16,
        min_dop_forsave: u8,
        gain_control: String,
        sig_process: String,
        spares: Vec<u8>,
        extension: String,
        noofreceivers: u8,
        incomplete_data: bool,
        incomplete_header: bool,
    ) -> Self {
        Self {
            site,
            datetime,
            source,
            filetype,
            ndops,
            nfreqs,
            nheights,
            minheight,
            maxheight,
            dheight,
            pps,
            npulses_avgd,
            dtime,
            base_thr100,
            noise_thr100,
            min_dop_forsave,
            gain_control,
            sig_process,
            spares,
            extension,
            noofreceivers,
            incomplete_data,
            incomplete_header,
        }
    }
}

impl Default for CADIheader {
    fn default() -> Self {
        Self {
            site: String::new(),
            datetime: PyTzDateTime(
                DateTime::from_timestamp(0, 0)
                    .unwrap()
                    .with_timezone((&UTC).into()),
            ),
            source: String::new(),
            filetype: String::new(),
            ndops: 0,
            nfreqs: 0,
            nheights: 0,
            minheight: 0,
            maxheight: 0,
            dheight: 0.0,
            pps: 0,
            npulses_avgd: 0,
            dtime: 0,
            base_thr100: 0,
            noise_thr100: 0,
            min_dop_forsave: 0,
            gain_control: String::new(),
            sig_process: String::new(),
            spares: vec![0u8; 11],
            extension: String::new(),
            noofreceivers: 0,
            incomplete_data: false,
            incomplete_header: false,
        }
    }
}
