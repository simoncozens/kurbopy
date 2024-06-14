use crate::cubicbez::CubicBez;
use crate::line::Line;
use crate::nearest::Nearest;
use crate::point::Point;
use crate::quadbez::QuadBez;
use kurbo::{
    ParamCurve, ParamCurveArclen, ParamCurveArea, ParamCurveCurvature, ParamCurveNearest,
    PathSeg as KPathSeg,
};
use pyo3::prelude::*;

#[pyclass(subclass, module = "kurbopy")]
#[derive(Clone, Debug)]
pub struct PathSeg(pub KPathSeg);

impl From<KPathSeg> for PathSeg {
    fn from(p: KPathSeg) -> Self {
        Self(p)
    }
}

#[pymethods]
impl PathSeg {
    fn as_line(&self) -> Option<Line> {
        if let KPathSeg::Line(l) = self.0 {
            Some(l.into())
        } else {
            None
        }
    }

    fn as_quad(&self) -> Option<QuadBez> {
        if let KPathSeg::Quad(l) = self.0 {
            Some(l.into())
        } else {
            None
        }
    }

    fn as_cubic(&self) -> Option<CubicBez> {
        if let KPathSeg::Cubic(l) = self.0 {
            Some(l.into())
        } else {
            None
        }
    }

    fn eval(&self, t: f64) -> Point {
        self.0.eval(t).into()
    }
    fn subsegment(&self, start: f64, end: f64) -> PathSeg {
        self.0.subsegment(start..end).into()
    }
    fn arclen(&self, accuracy: f64) -> f64 {
        self.0.arclen(accuracy)
    }
    fn inv_arclen(&self, arclen: f64, accuracy: f64) -> f64 {
        self.0.inv_arclen(arclen, accuracy)
    }
    fn signed_area(&self) -> f64 {
        self.0.signed_area()
    }
    fn nearest(&self, p: Point, accuracy: f64) -> Nearest {
        self.0.nearest(p.0, accuracy).into()
    }

    // Kurbo doesn't provide this because of the type system, but
    // we can!
    fn curvature(&self, t: f64) -> f64 {
        match self.0 {
            KPathSeg::Line(line) => line.curvature(t),
            KPathSeg::Quad(quad) => quad.curvature(t),
            KPathSeg::Cubic(cubic) => cubic.curvature(t),
        }
    }

    fn deriv(&self, py: Python) -> PyObject {
        match self.0 {
            KPathSeg::Line(line) => Line(line).deriv().into_py(py),
            KPathSeg::Quad(quad) => QuadBez(quad).deriv().into_py(py),
            KPathSeg::Cubic(cubic) => CubicBez(cubic).deriv().into_py(py),
        }
    }
}
