use pyo3::prelude::*;

use segul::helper::alphabet::DNA_STR_UPPERCASE;

pub(crate) fn register(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(alphabet, m)?)?;

    Ok(())
}

#[pyfunction]
fn alphabet() -> String {
    DNA_STR_UPPERCASE.to_string()
}
