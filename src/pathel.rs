use kurbo::PathEl as KPathEl;
use pyo3::prelude::*;

#[pyclass(subclass, module = "kurbopy")]
#[derive(Clone, Debug)]
pub struct PathEl(pub KPathEl);

impl From<KPathEl> for PathEl {
    fn from(p: KPathEl) -> Self {
        Self(p)
    }
}
