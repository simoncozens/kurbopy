use kurbo::PathEl as KPathEl;
use pyo3::prelude::*;

use crate::{impl_isfinitenan, point::Point};

#[pyclass(subclass, module = "kurbopy")]
#[derive(Clone, Debug)]
pub struct PathEl(pub KPathEl);

impl From<KPathEl> for PathEl {
    fn from(p: KPathEl) -> Self {
        Self(p)
    }
}

impl_isfinitenan!(PathEl);

#[pymethods]
impl PathEl {
    /// Get the end point of the path element, if it exists.
    pub fn end_point(&self) -> Option<Point> {
        self.0.end_point().map(Point)
    }
}