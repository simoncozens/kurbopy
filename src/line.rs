use crate::constpoint::ConstPoint;
use crate::nearest::Nearest;
use crate::point::Point;
use crate::vec2::Vec2;
use crate::{
    impl_isfinitenan, impl_paramcurve, impl_paramcurvearclen, impl_paramcurvearea, impl_paramcurvecurvature, impl_paramcurvederiv, impl_paramcurveextrema, impl_paramcurvenearest
};

use kurbo::{
    Line as KLine, ParamCurve, ParamCurveArclen, ParamCurveArea, ParamCurveCurvature,
    ParamCurveDeriv, ParamCurveExtrema, ParamCurveNearest,
};
use pyo3::prelude::*;

#[pyclass(subclass, module = "kurbopy")]
#[derive(Clone, Debug)]
/// A single line.
pub struct Line(pub KLine);

impl From<KLine> for Line {
    fn from(p: KLine) -> Self {
        Self(p)
    }
}
#[pymethods]
impl Line {
    #[new]
    fn __new__(p0: Point, p1: Point) -> Self {
        Line(KLine::new(p0.0, p1.0))
    }

    /// The length of the line.
    fn length(&self) -> f64 {
        self.0.length()
    }
    /// Computes the point where two lines, if extended to infinity, would cross
    fn crossing_point(&self, other: &Line) -> Option<Point> {
        self.0.crossing_point(other.0).map(|p| p.into())
    }

    #[getter]
    fn get_p0(&self) -> Point {
        self.0.p0.into()
    }
    #[getter]
    fn get_p1(&self) -> Point {
        self.0.p1.into()
    }
    #[setter]
    fn set_p0(&mut self, p0: Point) {
        self.0.p0 = p0.0;
    }
    #[setter]
    fn set_p1(&mut self, p1: Point) {
        self.0.p1 = p1.0;
    }

    #[allow(non_snake_case)]
    fn __add__(&self, rhs: Vec2) -> PyResult<Line> {
        let p: Line = (self.0 + rhs.0).into();
        Ok(p)
    }

    #[allow(non_snake_case)]
    fn __sub__(&self, rhs: Vec2) -> PyResult<Line> {
        let p: Line = (self.0 - rhs.0).into();
        Ok(p)
    }
}

impl_paramcurve!(Line);
impl_paramcurvearclen!(Line);
impl_paramcurvearea!(Line);
impl_paramcurvecurvature!(Line);
impl_paramcurveextrema!(Line);
impl_paramcurvenearest!(Line);
impl_isfinitenan!(Line);
impl_paramcurvederiv!(Line, ConstPoint);