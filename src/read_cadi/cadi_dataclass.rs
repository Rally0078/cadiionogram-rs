// Crate containing the data structures for holding CADI data and metadata, along with Python bindings
use chrono::DateTime;
use chrono_tz::UTC;
use std::collections::BTreeMap;
use pyo3::prelude::*;
use pyo3::IntoPyObjectExt;
use numpy::{PyArray1, PyArray2, IntoPyArray, PyArrayMethods};
use crate::pytzdatetime::PyTzDateTime;

#[pyclass(from_py_object)]
#[derive(Debug, Clone)]
pub struct Metadata {
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
    pub extension: String,
    #[pyo3(get, set)]
    pub noofreceivers: u8,
    #[pyo3(get, set, name = "timepartitions")]
    pub time_partitions: BTreeMap<String, usize>,
    #[pyo3(get, set, name = "incompletedata")]
    pub incomplete_data: bool,
    #[pyo3(get, set, name = "incompleteheader")]
    pub incomplete_header: bool,
}

#[pymethods]
impl Metadata {
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
            "extension" => self.extension.clone().into_py_any(py),
            "noofreceivers" => self.noofreceivers.into_py_any(py),
            "timepartitions" => self.time_partitions.clone().into_py_any(py),
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
            "extension" => self.extension = value.extract()?,
            "noofreceivers" => self.noofreceivers = value.extract()?,
            "timepartitions" => self.time_partitions = value.extract()?,
            "incompletedata" => self.incomplete_data = value.extract()?,
            "incompleteheader" => self.incomplete_header = value.extract()?,
            _ => return Err(pyo3::exceptions::PyKeyError::new_err(key.to_string())),
        }
        Ok(())
    }

    fn keys(&self) -> Vec<&'static str> {
        vec![
            "site", "datetime", "source", "filetype", "ndops", "nfreqs", "nheights",
            "minheight", "maxheight", "dheight", "pps", "npulses_avgd", "dtime",
            "extension", "noofreceivers", "timepartitions", "incompletedata", "incompleteheader",
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
            ("extension", self.extension.clone().into_py_any(py)?),
            ("noofreceivers", self.noofreceivers.into_py_any(py)?),
            ("timepartitions", self.time_partitions.clone().into_py_any(py)?),
            ("incompletedata", self.incomplete_data.into_py_any(py)?),
            ("incompleteheader", self.incomplete_header.into_py_any(py)?),
        ])
    }
}

impl Default for Metadata {
    fn default() -> Self {
        Self {
            site: String::new(),
            datetime: PyTzDateTime(
                DateTime::from_timestamp(0, 0)
                    .unwrap()
                    .with_timezone((&UTC).into())
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
            extension: String::new(),
            noofreceivers: 0,
            time_partitions: BTreeMap::new(),
            incomplete_data: false,
            incomplete_header: false,
        }
    }
}

#[pyclass]
pub struct CadiData {
    #[pyo3(get)]
    pub file_list: Vec<String>,
    #[pyo3(get)]
    pub metadata: Metadata,
    pub height: Vec<f32>,
    pub frequency: Vec<f32>,
    pub freqs: Vec<f32>,
    pub dop_shifts: Vec<f32>,
    pub complex_signal: Vec<i16>, // Flattened: [dopbin][receiver_re_im]
}

#[pymethods]
impl CadiData {
    #[getter]
    fn height<'py>(&self, py: Python<'py>) -> Bound<'py, PyArray1<f32>> {
        self.height.clone().into_pyarray(py)
    }

    #[getter]
    fn frequency<'py>(&self, py: Python<'py>) -> Bound<'py, PyArray1<f32>> {
        self.frequency.clone().into_pyarray(py)
    }

    #[getter]
    fn freqs<'py>(&self, py: Python<'py>) -> Bound<'py, PyArray1<f32>> {
        self.freqs.clone().into_pyarray(py)
    }

    #[getter]
    fn dop_shifts<'py>(&self, py: Python<'py>) -> Bound<'py, PyArray1<f32>> {
        self.dop_shifts.clone().into_pyarray(py)
    }

    #[getter]
    fn complex_signal<'py>(&self, py: Python<'py>) -> Bound<'py, PyArray2<i16>> {
        let n_receivers = self.metadata.noofreceivers as usize;
        let n_bins = self.height.len();
        let arr = self.complex_signal.clone().into_pyarray(py);
        arr.reshape([n_bins, n_receivers * 2]).unwrap()
    }

    fn __iter__<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
        let items = vec![
            self.file_list.clone().into_py_any(py)?,
            self.metadata.clone().into_py_any(py)?,
            self.height(py).into_any().into_py_any(py)?,
            self.frequency(py).into_any().into_py_any(py)?,
            self.freqs(py).into_any().into_py_any(py)?,
            self.dop_shifts(py).into_any().into_py_any(py)?,
            self.complex_signal(py).into_any().into_py_any(py)?,
        ];
        let list = pyo3::types::PyList::new(py, items)?;
        list.call_method0("__iter__")
    }
}

impl CadiData {
    pub fn empty(metadata: Metadata) -> Self {
        Self {
            file_list: Vec::new(),
            metadata,
            height: Vec::new(),
            frequency: Vec::new(),
            freqs: Vec::new(),
            dop_shifts: Vec::new(),
            complex_signal: Vec::new(),
        }
    }
}
