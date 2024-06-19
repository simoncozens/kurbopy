use kurbo::MinDistance as KMinDistance;
use pyo3::prelude::*;
#[pyclass(subclass, module = "kurbopy")]
pub struct MinDistance(KMinDistance);

impl From<KMinDistance> for MinDistance {
    fn from(p: KMinDistance) -> Self {
        Self(p)
    }
}

#[pymethods]
impl MinDistance {
    // getters and setters
    #[getter]
    fn get_distance(&self) -> f64 {
        self.0.distance
    }
    #[setter]
    fn set_distance(&mut self, value: f64) {
        self.0.distance = value;
    }
    #[getter]
    fn get_t1(&self) -> f64 {
        self.0.t1
    }
    #[setter]
    fn set_t1(&mut self, value: f64) {
        self.0.t1 = value;
    }
    #[getter]
    fn get_t2(&self) -> f64 {
        self.0.t2
    }
    #[setter]
    fn set_t2(&mut self, value: f64) {
        self.0.t2 = value;
    }
}
