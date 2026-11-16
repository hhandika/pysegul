use pyo3::prelude::*;

use segul_core::helper::alphabet::DNA_STR_UPPERCASE;

pub(crate) fn register(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(alphabet, m)?)?;

    Ok(())
}

/// Uppercase DNA characters, including IUPAC codes, gaps, and missing data.
#[pyfunction]
fn alphabet() -> String {
    DNA_STR_UPPERCASE.to_string()
}
