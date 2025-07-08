use pyo3::prelude::*;
use rosu_memory_lib::common::GameMode;

#[pyclass(name = "GameMode")]
#[derive(Clone)]
pub struct PyGameMode(pub(crate) GameMode);

#[pymethods]
impl PyGameMode {
    #[new]
    fn new(mode: i32) -> Self {
        PyGameMode(GameMode::from(mode))
    }

    fn __str__(&self) -> PyResult<String> {
        Ok(format!("{:?}", self.0))
    }
} 