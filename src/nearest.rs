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
