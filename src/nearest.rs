use kurbo::Nearest as KNearest;
use pyo3::prelude::*;
#[pyclass(subclass, module = "kurbopy")]
#[derive(Clone, Debug)]
pub struct Nearest {
    pub distance_sq: f64,
    pub t: f64,
}

impl From<KNearest> for Nearest {
    fn from(p: KNearest) -> Self {
        Self {
            distance_sq: p.distance_sq,
            t: p.t,
        }
    }
}

#[pymethods]
impl Nearest {
    // getters and setters
    fn get_distance_sq(&self) -> f64 {
        self.distance_sq
    }
    fn set_distance_sq(&mut self, value: f64) {
        self.distance_sq = value;
    }
    fn get_t(&self) -> f64 {
        self.t
    }
    fn set_t(&mut self, value: f64) {
        self.t = value;
    }

}
