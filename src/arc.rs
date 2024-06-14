use crate::nearest::Nearest;
use crate::point::Point;
use crate::quadbez::QuadBez;
use crate::rect::Rect;
use crate::vec2::Vec2;

use kurbo::{
    Arc as KArc, ParamCurve, ParamCurveArclen, ParamCurveArea, ParamCurveCurvature,
    ParamCurveDeriv, ParamCurveExtrema, ParamCurveNearest, Point as KPoint, Shape,
};
use pyo3::prelude::*;

#[derive(Clone, Debug)]
#[pyclass(subclass, module = "kurbopy")]
/// A single arc segment.
pub struct Arc(pub KArc);

impl From<KArc> for Arc {
    fn from(p: KArc) -> Self {
        Self(p)
    }
}

#[pymethods]
impl Arc {
    /// Create a new `Arc`.
    #[new]
    pub fn __new__(
        center: Point,
        radii: Vec2,
        start_angle: f64,
        sweep_angle: f64,
        x_rotation: f64,
    ) -> Self {
        Self(KArc::new(
            center.0,
            radii.0,
            start_angle,
            sweep_angle,
            x_rotation,
        ))
    }

    /// Converts an Arc into a series of cubic bezier segments.
    ///
    /// Closure will be invoked for each segment.
    pub fn to_cubic_beziers(&self, py: Python, tolerance: f64, fun: Py<PyAny>) {
        let callback = |s: KPoint, m: KPoint, e: KPoint| {
            let _ = fun
                .call1(py, (Point(s), Point(m), Point(e)))
                .map_err(|e| e.restore(py));
        };
        self.0.to_cubic_beziers(tolerance, callback)
    }

    /// The area of the arc.
    ///
    /// Note: shape isn't closed so area is not well defined.
    fn area(&self) -> f64 {
        self.0.area()
    }

    /// The perimeter of the arc.
    ///
    /// For we just approximate by using the bezier curve representation.
    fn perimeter(&self, accuracy: f64) -> f64 {
        self.0.perimeter(accuracy)
    }

    ///The winding number of a point.
    ///
    /// Note: shape isn't closed, so a point's winding number is not well defined.
    fn winding(&self, pt: Point) -> i32 {
        self.0.winding(pt.0)
    }

    /// The smallest rectangle that encloses the shape.
    fn bounding_box(&self) -> Rect {
        self.0.bounding_box().into()
    }
}
