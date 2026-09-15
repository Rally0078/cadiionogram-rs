use numpy::{IntoPyArray, PyArray1};
use pyo3::IntoPyObjectExt;
use pyo3::prelude::*;
use serde::Serialize;
use std::collections::BTreeMap;

/// Contains the CADI frequency bin data from a given `mdX(X=1,2,3,4)` file.
///
/// timepartitions : `dict`
///     The timestamps and the cumulative length of each timestamp's observation data is given in `timepartitions`.
///
/// frequency : `numpy.ndarray`
///     Frequencies in Hz from all the observations in the file.
///
/// frebins_gain_flag : `numpy.ndarray`
///     Contains the gain flag values of all the observations.
///
/// frebins_noise_flag : `numpy.ndarray`
///     Contains the noise flag values of all the observations.
///
/// frebins_noise_power10 : `numpy.ndarray`
///     Contains the scaled noise power10 values of all the observations.
///
#[pyclass(from_py_object)]
#[derive(Debug, Clone, Serialize)]
pub struct CADIfreqbin {
    #[pyo3(get, set, name = "timepartitions")]
    pub timepartitions: BTreeMap<String, usize>,
    pub frebins_gain_flag: Vec<u8>,
    pub frequency: Vec<f32>,
    pub frebins_noise_flag: Vec<u8>,
    pub frebins_noise_power10: Vec<u16>,
}

impl CADIfreqbin {
    pub fn empty() -> Self {
        Self {
            timepartitions: BTreeMap::new(),
            frequency: Vec::new(),
            frebins_gain_flag: Vec::new(),
            frebins_noise_flag: Vec::new(),
            frebins_noise_power10: Vec::new(),
        }
    }
}

#[pymethods]
impl CADIfreqbin {
    #[getter]
    fn frequency<'py>(&self, py: Python<'py>) -> Bound<'py, PyArray1<f32>> {
        self.frequency.clone().into_pyarray(py)
    }

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
            self.frequency(py).into_any().into_py_any(py)?,
            self.frebins_gain_flag(py).into_any().into_py_any(py)?,
            self.frebins_noise_flag(py).into_any().into_py_any(py)?,
            self.frebins_noise_power10(py).into_any().into_py_any(py)?,
        ];
        let list = pyo3::types::PyList::new(py, items)?;
        list.call_method0("__iter__")
    }

    #[new]
    pub fn new(
        timepartitions: BTreeMap<String, usize>,
        frequency: Vec<f32>,
        frebins_gain_flag: Vec<u8>,
        frebins_noise_flag: Vec<u8>,
        frebins_noise_power10: Vec<u16>,
    ) -> Self {
        Self {
            timepartitions,
            frequency,
            frebins_gain_flag,
            frebins_noise_flag,
            frebins_noise_power10,
        }
    }
}
