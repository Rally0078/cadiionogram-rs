mod read_cadi;
mod siteinfo;

use pyo3::prelude::*;
use crate::read_cadi::{MDReader, CadiData, Metadata};
use std::path::Path;

#[pyfunction]
fn read_raw_data(filename: String) -> PyResult<CadiData> {
    let path = Path::new(&filename);
    MDReader::read_raw_data(path)
        .map_err(|e| pyo3::exceptions::PyIOError::new_err(e.to_string()))
}

#[pymodule]
fn mdreader_rs(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(read_raw_data, m)?)?;
    m.add_class::<CadiData>()?;
    m.add_class::<Metadata>()?;
    Ok(())
}
