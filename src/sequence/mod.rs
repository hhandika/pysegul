mod addition;
mod extract;
mod filter;
mod id;
mod remove;
mod rename;
mod translate;
mod transpose;

use addition::SequenceAddition;
use extract::SequenceExtraction;
use filter::SequenceFiltering;
use id::IDExtraction;
use pyo3::prelude::*;
use remove::SequenceRemoval;
use rename::SequenceRenaming;
use translate::SequenceTranslation;
use transpose::SequenceTranspose;

pub(crate) fn register(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<IDExtraction>()?;
    m.add_class::<SequenceAddition>()?;
    m.add_class::<SequenceExtraction>()?;
    m.add_class::<SequenceFiltering>()?;
    m.add_class::<SequenceRemoval>()?;
    m.add_class::<SequenceRenaming>()?;
    m.add_class::<SequenceTranslation>()?;
    m.add_class::<SequenceTranspose>()?;
    Ok(())
}
