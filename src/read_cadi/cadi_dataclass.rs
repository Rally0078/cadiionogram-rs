// Crate containing the data structures for holding CADI data and metadata, along with Python bindings
//use crate::read_cadi::{cadi_dopbins, cadi_freqbins, cadi_header};
use crate::read_cadi::CADIdopbin;
use crate::read_cadi::CADIfreqbin;
use crate::read_cadi::CADIheader;
use pyo3::prelude::*;

#[pyclass]
pub struct CADIdata {
    #[pyo3(get)]
    pub file_list: Vec<String>,
    #[pyo3(get)]
    pub metadata: CADIheader,
    #[pyo3(get)]
    pub freq_list: Vec<f32>,
    #[pyo3(get)]
    pub dopbins: CADIdopbin,
    #[pyo3(get)]
    pub freqbins: CADIfreqbin,
}

impl CADIdata {
    pub fn empty(metadata: CADIheader) -> Self {
        let n_receivers = metadata.noofreceivers;
        Self {
            file_list: Vec::new(),
            metadata,
            freq_list: Vec::new(),
            dopbins: CADIdopbin::empty(n_receivers),
            freqbins: CADIfreqbin::empty(),
        }
    }
}
