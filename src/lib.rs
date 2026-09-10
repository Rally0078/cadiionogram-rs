mod pytzdatetime;
pub mod read_cadi;
pub mod siteinfo;
use crate::read_cadi::{CADIdata, CADIdopbin, CADIfreqbin, CADIheader, MDReader};
use pyo3::prelude::*;
use std::path::PathBuf;

#[pyfunction]
fn read_raw_data(filename: PathBuf) -> PyResult<CADIdata> {
    MDReader::read_raw_data(&filename)
        .map_err(|e| pyo3::exceptions::PyIOError::new_err(e.to_string()))
}

#[pymodule]
fn mdxreader_rs(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(read_raw_data, m)?)?;
    m.add_class::<CADIdata>()?;
    m.add_class::<CADIheader>()?;
    m.add_class::<CADIfreqbin>()?;
    m.add_class::<CADIdopbin>()?;
    Ok(())
}
