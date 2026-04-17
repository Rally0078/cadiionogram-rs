mod read_cadi;
mod siteinfo;
mod pytzdatetime;
use pyo3::prelude::*;
use crate::read_cadi::{MDReader, CadiData, Metadata};
use std::path::PathBuf;

#[pyfunction]
fn read_raw_data(filename: PathBuf) -> PyResult<CadiData> {
    MDReader::read_raw_data(&filename)
        .map_err(|e| pyo3::exceptions::PyIOError::new_err(e.to_string()))
}

#[pymodule]
fn mdreader_rs(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(read_raw_data, m)?)?;
    m.add_class::<CadiData>()?;
    m.add_class::<Metadata>()?;
    Ok(())
}
