//! Load the JSON reports written by the summary classes into Python dicts.

use std::path::Path;

use pyo3::{prelude::*, types::PyModule};
use segul_core::stats::{
    align_report::AlignReport,
    contig_report::ContigReport,
    distance_report::DistanceReport,
    fastq::{
        qc::{overall_status, qc_checks},
        report::ReadReport,
    },
};
use serde::Serialize;
use serde_json::{json, Value};

use crate::common::to_py_err;

pub(crate) fn register(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(load_align_report, m)?)?;
    m.add_function(wrap_pyfunction!(load_contig_report, m)?)?;
    m.add_function(wrap_pyfunction!(load_distance_report, m)?)?;
    m.add_function(wrap_pyfunction!(load_read_report, m)?)?;
    Ok(())
}

/// Load an alignment summary report (`alignment-report.json`) as a dict.
#[pyfunction]
fn load_align_report(py: Python<'_>, path: &str) -> PyResult<Py<PyAny>> {
    let report = AlignReport::from_path(Path::new(path)).map_err(to_py_err)?;
    to_py(py, &report)
}

/// Load a contig summary report (`contig-report.json`) as a dict.
#[pyfunction]
fn load_contig_report(py: Python<'_>, path: &str) -> PyResult<Py<PyAny>> {
    let report = ContigReport::from_path(Path::new(path)).map_err(to_py_err)?;
    to_py(py, &report)
}

/// Load a genetic distance report (`distance-report.json`) as a dict.
#[pyfunction]
fn load_distance_report(py: Python<'_>, path: &str) -> PyResult<Py<PyAny>> {
    let report = DistanceReport::from_path(Path::new(path)).map_err(to_py_err)?;
    to_py(py, &report)
}

/// Load a read summary report (`read-report.json`, complete mode) as a dict.
/// Each sample also has `qc`, the read quality checks, and `qc_status`,
/// the worst status among them ('Pass', 'Warn', or 'Fail').
#[pyfunction]
fn load_read_report(py: Python<'_>, path: &str) -> PyResult<Py<PyAny>> {
    let report = ReadReport::from_path(Path::new(path)).map_err(to_py_err)?;
    let mut value = serde_json::to_value(&report).map_err(to_py_err)?;
    if let Some(samples) = value["samples"].as_array_mut() {
        for (sample, sample_report) in samples.iter_mut().zip(&report.samples) {
            let checks = qc_checks(sample_report);
            let qc: Vec<Value> = checks
                .iter()
                .map(|check| {
                    json!({
                        "name": check.name,
                        "value": check.value,
                        "unit": check.rule.unit,
                        "status": check.status.label(),
                        "description": check.rule.description,
                    })
                })
                .collect();
            sample["qc"] = Value::Array(qc);
            sample["qc_status"] = json!(overall_status(&checks).label());
        }
    }
    to_py(py, &value)
}

// Round trip through Python's json module, so the result holds only
// plain dicts, lists, and numbers.
fn to_py<T: Serialize>(py: Python<'_>, report: &T) -> PyResult<Py<PyAny>> {
    let text = serde_json::to_string(report).map_err(to_py_err)?;
    let json = PyModule::import(py, "json")?;
    Ok(json.call_method1("loads", (text,))?.unbind())
}
