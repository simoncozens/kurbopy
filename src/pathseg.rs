use crate::pathel::PathEl;
use crate::{impl_paramcurve, impl_paramcurvearclen, impl_paramcurvearea, impl_paramcurveextrema, impl_paramcurvenearest, impl_shape_no_bounding_box};
use crate::{cubicbez::CubicBez, impl_isfinitenan};
use crate::line::Line;
use crate::mindistance::MinDistance;
use crate::nearest::Nearest;
use crate::point::Point;
use crate::quadbez::QuadBez;
use kurbo::{
    ParamCurve, ParamCurveArclen, ParamCurveArea, ParamCurveCurvature, ParamCurveExtrema, ParamCurveNearest,
    PathSeg as KPathSeg, LineIntersection as KLineIntersection
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

    /// Get the [`PathEl`] that is equivalent to discarding the segment start point.
    pub fn as_path_el(&self) -> PathEl {
        PathEl(self.0.as_path_el())
    }

    /// Returns a new `PathSeg` describing the same path as `self`, but with
    /// the points reversed.
    pub fn reverse(&self) -> PathSeg {
        PathSeg(self.0.reverse())
    }
    /// Convert this segment to a cubic bezier.
    pub fn to_cubic(&self) -> CubicBez {
        CubicBez(self.0.to_cubic())
    }

    /// Compute intersections against a line.
    ///
    /// Returns a vector of the intersections. For each intersection,
    /// the `t` value of the segment and line are given.
    ///
    /// Note: This test is designed to be inclusive of points near the endpoints
    /// of the segment. This is so that testing a line against multiple
    /// contiguous segments of a path will be guaranteed to catch at least one
    /// of them. In such cases, use higher level logic to coalesce the hits
    /// (the `t` value may be slightly outside the range of 0..1).
    pub fn intersect_line(&self, line: Line) -> Vec<LineIntersection> {
        self.0.intersect_line(line.0).into_iter().map(|x| x.into()).collect()
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

    /// Minimum distance between two [`PathSeg`]s.
    ///
    /// Returns a tuple of the distance, the path time `t1` of the closest point
    /// on the first `PathSeg`, and the path time `t2` of the closest point on the
    /// second `PathSeg`.
    fn min_dist(&self, other: &PathSeg, accuracy: f64) -> MinDistance {
        self.0.min_dist(other.0, accuracy).into()
    }
}

impl_paramcurve!(PathSeg);
impl_paramcurvearclen!(PathSeg);
impl_paramcurvearea!(PathSeg);
impl_paramcurveextrema!(PathSeg);
impl_paramcurvenearest!(PathSeg);
impl_isfinitenan!(PathSeg);
impl_shape_no_bounding_box!(PathSeg);



#[pyclass(subclass, module = "kurbopy")]
#[derive(Clone, Debug)]
pub struct LineIntersection(pub KLineIntersection);

impl From<KLineIntersection> for LineIntersection {
    fn from(p: KLineIntersection) -> Self {
        Self(p)
    }
}

#[pymethods]
impl LineIntersection {
    #[getter]
    fn line_t(&self) -> f64 {
        self.0.line_t
    }
    #[getter]
    fn segment_t(&self) -> f64 {
        self.0.segment_t
    }
}
impl_isfinitenan!(LineIntersection);