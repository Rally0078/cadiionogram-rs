use numpy::{IntoPyArray, PyArray1};
use pyo3::IntoPyObjectExt;
use pyo3::prelude::*;
use serde::Serialize;
use std::collections::BTreeMap;

#[pyclass(from_py_object)]
#[derive(Debug, Clone, Serialize)]
pub struct CADIfreqbin {
    #[pyo3(get, set, name = "timepartitions")]
    pub timepartitions: BTreeMap<String, usize>,
    pub frebins_gain_flag: Vec<u8>,
    pub frebins_noise_flag: Vec<u8>,
    pub frebins_noise_power10: Vec<u16>,
}

impl CADIfreqbin {
    pub fn empty() -> Self {
        Self {
            timepartitions: BTreeMap::new(),
            frebins_gain_flag: Vec::new(),
            frebins_noise_flag: Vec::new(),
            frebins_noise_power10: Vec::new(),
        }
    }
}

#[pymethods]
impl CADIfreqbin {
    #[getter]
    fn frebins_gain_flag<'py>(&self, py: Python<'py>) -> Bound<'py, PyArray1<u8>> {
        self.frebins_gain_flag.clone().into_pyarray(py)
    }

    #[getter]
    fn frebins_noise_flag<'py>(&self, py: Python<'py>) -> Bound<'py, PyArray1<u8>> {
        self.frebins_noise_flag.clone().into_pyarray(py)
    }

    #[getter]
    fn frebins_noise_power10<'py>(&self, py: Python<'py>) -> Bound<'py, PyArray1<u16>> {
        self.frebins_noise_power10.clone().into_pyarray(py)
    }
    fn __iter__<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
        let items = vec![
            self.frebins_gain_flag(py).into_any().into_py_any(py)?,
            self.frebins_noise_flag(py).into_any().into_py_any(py)?,
            self.frebins_noise_power10(py).into_any().into_py_any(py)?,
        ];
        let list = pyo3::types::PyList::new(py, items)?;
        list.call_method0("__iter__")
    }
}
