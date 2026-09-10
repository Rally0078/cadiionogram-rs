use numpy::{IntoPyArray, PyArray1, PyArray2, PyArrayMethods};
use pyo3::IntoPyObjectExt;
use pyo3::prelude::*;
use serde::Serialize;
use std::collections::BTreeMap;

#[pyclass(from_py_object)]
#[derive(Debug, Clone, Serialize)]
pub struct CADIdopbin {
    #[pyo3(get, set, name = "timepartitions")]
    pub timepartitions: BTreeMap<String, usize>,
    #[pyo3(get, set, name = "nreceivers")]
    pub nreceivers: u8,
    pub height: Vec<f32>,
    pub frequency: Vec<f32>,
    pub freqs: Vec<f32>,
    pub dop_shifts: Vec<f32>,
    pub signals: Vec<i16>, // Flattened: [dopbin][receiver_re_im]
}

impl CADIdopbin {
    pub fn empty(n_receivers: u8) -> Self {
        Self {
            nreceivers: n_receivers,
            timepartitions: BTreeMap::new(),
            height: Vec::new(),
            frequency: Vec::new(),
            freqs: Vec::new(),
            dop_shifts: Vec::new(),
            signals: Vec::new(),
        }
    }
}

#[pymethods]
impl CADIdopbin {
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
    fn signals<'py>(&self, py: Python<'py>) -> Bound<'py, PyArray2<i16>> {
        let n_receivers = self.nreceivers as usize;
        let n_bins = self.height.len();
        let arr = self.signals.clone().into_pyarray(py);
        arr.reshape([n_bins, n_receivers * 2]).unwrap()
    }

    fn __iter__<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
        let items = vec![
            self.height(py).into_any().into_py_any(py)?,
            self.frequency(py).into_any().into_py_any(py)?,
            self.freqs(py).into_any().into_py_any(py)?,
            self.dop_shifts(py).into_any().into_py_any(py)?,
            self.signals(py).into_any().into_py_any(py)?,
        ];
        let list = pyo3::types::PyList::new(py, items)?;
        list.call_method0("__iter__")
    }
}
