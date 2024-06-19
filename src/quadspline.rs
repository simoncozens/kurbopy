use kurbo::QuadSpline as KQuadSpline;
use crate::{point::Point, quadbez::QuadBez};
use pyo3::prelude::*;

#[derive(Clone, Debug)]
#[pyclass(subclass, module = "kurbopy")]
/// A quadratic Bézier spline in B-spline format.
pub struct QuadSpline(pub KQuadSpline);

impl From<KQuadSpline> for QuadSpline {
    fn from(p: KQuadSpline) -> Self {
        Self(p)
    }
}
#[pymethods]
impl QuadSpline {
    #[new]
    fn __new__(points: Vec<Point>) -> Self {
        QuadSpline(KQuadSpline::new(points.iter().map(|p| p.0).collect()))
    }
    /// Return the spline’s control `Point`s.
    fn points(&self) -> Vec<Point> {
        self.0.points().iter().map(|p| Point(*p)).collect()
    }
    /// Return an iterator over the implied `QuadBez`` sequence.
    fn to_quads(&self) -> Vec<QuadBez> {
        self.0.to_quads().map(QuadBez).collect()
    }
}
