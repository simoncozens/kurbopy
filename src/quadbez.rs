use crate::{impl_paramcurve, impl_paramcurvearclen, impl_paramcurvearea, impl_paramcurvecurvature, impl_paramcurvederiv, impl_paramcurveextrema, impl_paramcurvenearest, impl_shape_no_bounding_box};
use crate::{cubicbez::CubicBez, impl_isfinitenan};
use crate::line::Line;
use crate::nearest::Nearest;
use crate::point::Point;
use kurbo::{
    ParamCurve, ParamCurveArclen, ParamCurveArea, ParamCurveCurvature, ParamCurveDeriv,
    ParamCurveExtrema, ParamCurveNearest, QuadBez as KQuadBez,
};
use pyo3::prelude::*;

#[pyclass(subclass, module = "kurbopy")]
#[derive(Clone, Debug)]
/// A single quadratic Bézier segment.
pub struct QuadBez(pub KQuadBez);

impl From<KQuadBez> for QuadBez {
    fn from(p: KQuadBez) -> Self {
        Self(p)
    }
}
#[pymethods]
impl QuadBez {
    #[new]
    fn __new__(p0: Point, p1: Point, p2: Point) -> Self {
        QuadBez(KQuadBez::new(p0.0, p1.0, p2.0))
    }

    /// Raise the order by 1.
    ///
    /// Returns a cubic Bézier segment that exactly represents this quadratic.
    fn raise(&self) -> CubicBez {
        self.0.raise().into()
    }

    #[getter]
    fn get_p0(&self) -> Point {
        self.0.p0.into()
    }
    #[getter]
    fn get_p1(&self) -> Point {
        self.0.p1.into()
    }
    #[getter]
    fn get_p2(&self) -> Point {
        self.0.p2.into()
    }
    #[setter]
    fn set_p0(&mut self, p0: Point) {
        self.0.p0 = p0.0;
    }
    #[setter]
    fn set_p1(&mut self, p1: Point) {
        self.0.p1 = p1.0;
    }
    #[setter]
    fn set_p2(&mut self, p2: Point) {
        self.0.p2 = p2.0;
    }
}

impl_isfinitenan!(QuadBez);
impl_paramcurve!(QuadBez);
impl_paramcurvearclen!(QuadBez);
impl_paramcurvearea!(QuadBez);
impl_paramcurvecurvature!(QuadBez);
impl_paramcurvederiv!(QuadBez, Line);
impl_paramcurveextrema!(QuadBez);
impl_paramcurvenearest!(QuadBez);
impl_shape_no_bounding_box!(QuadBez);
