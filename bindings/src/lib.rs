use pyo3::{prelude::*, pyclass, PyNativeType};
use augmented_interval_list::{interval, ailist};
/// Formats the sum of two numbers as string.

#[pyclass(name = "AIList")]
struct PyAIList{
    ailist: ailist::AIList
}

#[pymethods]
impl PyAIList {
    #[new]
    fn new(py_interval_list: Vec<PyRef<PyInterval>>, minimum_coverage_length: Option<usize>) -> PyAIList {
            let minimum_coverage_length: usize = minimum_coverage_length.unwrap_or(3);
            let interval_list: Vec<interval::Interval> = py_interval_list
                .into_iter()
                .map(|x| interval::Interval{start: x.start, end: x.end}).collect();
            let ailist = ailist::AIList::new(interval_list, minimum_coverage_length);
            PyAIList { ailist }
    }
    fn query(&self, py_interval: &PyInterval) -> Vec<PyInterval> {
        let interval: interval::Interval = interval::Interval{ start: py_interval.start, end: py_interval.end };
        self.ailist.query(&interval).into_iter().map(|x| PyInterval{start: x.start, end: x.end}).collect()
    }
}

#[pyclass(name = "Interval")]
struct PyInterval{
    #[pyo3(get, set)]
    pub start: u32,
    #[pyo3(get, set)]
    pub end: u32
}

#[pymethods]
impl PyInterval{
    #[new]
    fn new(start: u32, end: u32) -> PyInterval {
        PyInterval { start, end }
    }
    fn __repr__(&self) -> String {
        format!("Interval({}, {})", self.start, self.end)
    }
}

/// A Python module implemented in Rust.
#[pymodule]
fn bindings(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<PyInterval>()?;
    m.add_class::<PyAIList>()?;
    Ok(())
}
