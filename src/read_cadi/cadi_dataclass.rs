// Crate containing the data structures for holding CADI data and metadata, along with Python bindings
//use crate::read_cadi::{cadi_dopbins, cadi_freqbins, cadi_header};
use crate::read_cadi::CADIdopbin;
use crate::read_cadi::CADIfreqbin;
use crate::read_cadi::CADIheader;
use pyo3::prelude::*;

///
/// Contains the CADI output data from a given `mdX(X=1,2,3,4)` file.
///
/// file: `List[str]`
///    List containing the name of the file.
///
/// metadata: `CADIheader`
///    An object containing metadata of the observations. Contains header info stored in the mdx file.
///
/// freq_list: `npt.NDArray[np.float32]`
///    A 1D array of frequencies in Hz, the unique values of the frequencies in the `freqbins` and `dopbins`.
///
/// freqbins: `CADIfreqbin`
///     An object containing the CADI frequency bin data.
///
/// dopbins: `CADIdopbin`
///     An object containing the CADI doppler bin data.
///
#[pyclass(dict)]
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

#[pymethods]
impl CADIdata {
    #[new]
    pub fn new(
        file_list: Vec<String>,
        metadata: CADIheader,
        freq_list: Vec<f32>,
        dopbins: CADIdopbin,
        freqbins: CADIfreqbin,
    ) -> Self {
        Self {
            file_list,
            metadata,
            freq_list,
            dopbins,
            freqbins,
        }
    }
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
