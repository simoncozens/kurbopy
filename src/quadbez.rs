use crate::cubicbez::CubicBez;
use crate::line::Line;
use crate::point::Point;
use crate::rect::Rect;
use kurbo::ParamCurveArclen;
use kurbo::{
    ParamCurve, ParamCurveArea, ParamCurveCurvature, ParamCurveDeriv, ParamCurveExtrema,
    ParamCurveNearest, QuadBez as KQuadBez, Shape,
};
use pyo3::prelude::*;

#[pyclass(subclass)]
#[derive(Clone, Debug)]
/// A single quadratic Bézier segment.
#[pyo3(text_signature = "(p0, p1, p2)")]
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

    /// Is this cubic Bezier curve finite?
    fn is_finite(&self) -> bool {
        self.0.is_finite()
    }
    /// Is this cubic Bezier curve NaN?
    fn is_nan(&self) -> bool {
        self.0.is_nan()
    }

    /// Evaluate the curve at parameter `t`.
    ///
    /// Generally `t` is in the range [0..1].
    #[pyo3(text_signature = "($self, t)")]
    fn eval(&self, t: f64) -> Point {
        self.0.eval(t).into()
    }
    /// The start point.
    fn start(&self) -> Point {
        self.0.start().into()
    }
    /// The end point.
    fn end(&self) -> Point {
        self.0.end().into()
    }
    /// Get a subsegment of the curve for the given parameter range.
    #[pyo3(text_signature = "($self, (t0,t1))")]
    fn subsegment(&self, range: (f64, f64)) -> Self {
        self.0.subsegment(range.0..range.1).into()
    }

    /// The arc length of the curve.
    ///
    /// The result is accurate to the given accuracy (subject to
    /// roundoff errors for ridiculously low values). Compute time
    /// may vary with accuracy, if the curve needs to be subdivided.
    #[pyo3(text_signature = "($self, accuracy)")]
    fn arclen(&self, accuracy: f64) -> f64 {
        self.0.arclen(accuracy)
    }

    /// Solve for the parameter that has the given arc length from the start.
    ///
    /// This implementation uses the IPT method, as provided by
    /// [`common::solve_itp`]. This is as robust as bisection but
    /// typically converges faster. In addition, the method takes
    /// care to compute arc lengths of increasingly smaller segments
    /// of the curve, as that is likely faster than repeatedly
    /// computing the arc length of the segment starting at t=0.
    #[pyo3(text_signature = "($self, arclen, accuracy)")]
    fn inv_arclen(&self, arclen: f64, accuracy: f64) -> f64 {
        self.0.inv_arclen(arclen, accuracy)
    }

    /// Compute the signed area under the curve.
    ///
    /// For a closed path, the signed area of the path is the sum of signed
    /// areas of the segments. This is a variant of the "shoelace formula."
    /// See:
    /// _<https://github.com/Pomax/bezierinfo/issues/44> and
    /// _<http://ich.deanmcnamee.com/graphics/2016/03/30/CurveArea.html>
    ///
    /// This can be computed exactly for Béziers thanks to Green's theorem,
    /// and also for simple curves such as circular arcs. For more exotic
    /// curves, it's probably best to subdivide to cubics. We leave that
    /// to the caller, which is why we don't give an accuracy param here.
    fn signed_area(&self) -> f64 {
        self.0.signed_area()
    }

    /// Find the position on the curve that is nearest to the given point.
    ///
    /// This returns a tuple ``(t, distance_sq)`` where ``t`` is
    /// the position on the curve of the nearest point, as a parameter, and
    /// ``distance_sq`` is the square of the distance from the nearest position on the curve
    /// to the given point.
    #[pyo3(text_signature = "($self, point, accuracy)")]
    fn nearest(&self, p: Point, accuracy: f64) -> (f64, f64) {
        let n = self.0.nearest(p.0, accuracy);
        (n.t, n.distance_sq)
    }

    /// Compute the signed curvature at parameter `t`.
    #[pyo3(text_signature = "($self, t)")]
    fn curvature(&self, t: f64) -> f64 {
        self.0.curvature(t)
    }

    /// Compute the extrema of the curve.
    ///
    /// Only extrema within the interior of the curve count.
    ///
    /// The extrema should be reported in increasing parameter order.
    fn extrema(&self) -> Vec<f64> {
        self.0.extrema().to_vec()
    }

    /// The derivative of the curve.
    ///
    /// Note that the type of the return value is somewhat inaccurate, as
    /// the derivative of a curve (mapping of param to point) is a mapping
    /// of param to vector. We choose to accept this rather than have a
    /// more complex type scheme.
    fn deriv(&self) -> Line {
        self.0.deriv().into()
    }

    /// Compute the signed area under the curve.
    ///
    /// For a closed path, the signed area of the path is the sum of signed
    /// areas of the segments. This is a variant of the "shoelace formula."
    /// See:
    /// <https://github.com/Pomax/bezierinfo/issues/44> and
    /// <http://ich.deanmcnamee.com/graphics/2016/03/30/CurveArea.html>
    ///
    /// This can be computed exactly for Béziers thanks to Green's theorem,
    /// and also for simple curves such as circular arcs. For more exotic
    /// curves, it's probably best to subdivide to cubics. We leave that
    /// to the caller, which is why we don't give an accuracy param here.
    fn area(&self) -> f64 {
        self.0.area()
    }

    /// Total length of perimeter.
    #[pyo3(text_signature = "($self, accuracy)")]
    fn perimeter(&self, accuracy: f64) -> f64 {
        self.0.perimeter(accuracy)
    }

    /// The winding number of a point.
    ///
    /// This method only produces meaningful results with closed shapes.
    ///
    /// The sign of the winding number is consistent with that of ``area``,
    /// meaning it is +1 when the point is inside a positive area shape
    /// and -1 when it is inside a negative area shape. Of course, greater
    /// magnitude values are also possible when the shape is more complex.
    #[pyo3(text_signature = "($self, pt)")]
    fn winding(&self, pt: Point) -> i32 {
        self.0.winding(pt.0)
    }

    /// The smallest rectangle that encloses the shape.
    fn bounding_box(&self) -> Rect {
        kurbo::Shape::bounding_box(&self.0).into()
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
