use pyo3::prelude::*;
use rosu_memory_lib::reader::{structs::State};
use rosu_mem::process::Process;

#[pyclass]
pub struct PyProcess(pub Process);

#[pymethods]
impl PyProcess {
    #[new]
    fn new() -> Self {
        unimplemented!("Cannot create Process directly")
    }
}

#[pyclass]
pub struct PyState(pub State);

#[pymethods]
impl PyState {
    #[new]
    fn new() -> Self {
        unimplemented!("Cannot create State directly")
    }
}