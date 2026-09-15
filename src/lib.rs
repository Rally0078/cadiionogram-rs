mod pytzdatetime;
pub mod read_cadi;
pub mod siteinfo;
use crate::read_cadi::{CADIdata, CADIdopbin, CADIfreqbin, CADIheader, MDReader};
use pyo3::prelude::*;
use std::path::PathBuf;

///
/// Read CADI ionogram data from mdx binary formats(x=1,2,3,4).
///
/// Parameters
/// ----------
/// filename : `Path`
///     Location of the mdx file to parse.
///
/// Returns
/// ----------
/// Returns multiple values in a `CADIdata` object as follows, where the arrays can be partitioned by the timepartitions provided in the corresponding data bins.
///
/// file_list : `List[str]`
///     List containing the name of the file.
///
/// metadata : `CADIheader`
///     An object containing metadata of the observations. Contains header info stored in the mdx file.
///
/// freqbins : `CADIfreqbin`
///     An object containing the CADI frequency bin data.
///
/// dopbins: `CADIdopbin`
///     An object containing the CADI doppler bin data.
///
/// Examples
/// --------
/// Read one md4 file from current directory
///
/// >>> output = MDreader.read_raw_data(Path('./input.md4'))
/// >>> files_list = output.file_list
/// >>> metadata = output.metadata
/// >>> heights = output.dopbins.height
/// >>> frequencies = output.dopbins.frequency
/// >>> freq_list = output.dopbins.freq_list
/// >>> dop_shifts = output.dopbins.dop_shifts
/// >>> complex_signal = output.dopbins.complex_signal
/// >>> frebins_noise_power10 = output.freqbins.frebins_noise_power10
///
#[pyfunction]
fn read_raw_data(filename: PathBuf) -> PyResult<CADIdata> {
    MDReader::read_raw_data(&filename)
        .map_err(|e| pyo3::exceptions::PyIOError::new_err(e.to_string()))
}

///   
/// MDx binary format ionogram parser. Contains the static method `read_raw_data` to read ionogram data from mdx file.
///
#[pymodule]
fn mdxreader_rs(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(read_raw_data, m)?)?;
    m.add_class::<CADIdata>()?;
    m.add_class::<CADIheader>()?;
    m.add_class::<CADIfreqbin>()?;
    m.add_class::<CADIdopbin>()?;
    Ok(())
}
