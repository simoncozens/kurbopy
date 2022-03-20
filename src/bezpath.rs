use crate::line::Line;
use crate::point::Point;
use crate::rect::Rect;
use core::cmp::Ordering;
use kurbo::{BezPath as KBezPath, CubicBez};
use kurbo::{ParamCurve, PathEl, PathSeg, Shape};
use pyo3::prelude::*;

#[pyclass(subclass)]
#[derive(Clone, Debug)]
/// A Bézier path.
///
/// These docs assume basic familiarity with Bézier curves; for an
/// introduction, see Pomax's wonderful `A Primer on Bézier Curves`_.
///
/// This path can contain lines, quadratics ([``QuadBez``]) and cubics
/// ([``CubicBez``]), and may contain multiple subpaths.
///
/// Elements and Segments
/// =====================
///
/// A Bézier path can be represented in terms of either 'elements'
/// ([``PathEl``]) or 'segments' ([``PathSeg``]). Elements map closely to
/// how Béziers are generally used in PostScript-style drawing APIs; they
/// can be thought of as instructions for drawing the path. Segments more
/// directly describe the path itself, with each segment being an
/// independent line or curve.
///
/// These different representations are useful in different contexts. For
/// tasks like drawing, elements are a natural fit, but when doing
/// hit-testing or subdividing, we need to have access to the segments.
///
/// ::
///
///    from kurbopy import BezPath, Rect, Shape, Vec2, Point
///    accuracy = 0.1
///    rect = Rect(Point(0, 0), Point(10, 10))
///    path1 = rect.to_path(accuracy)
///
///    # extend a path with another path:
///    path = rect.to_path(accuracy)
///    shifted_rect = rect + Vec2(5.0, 10.0)
///    path.extend(shifted_rect.to_path(accuracy))
///
///
/// Advanced functionality
/// ======================
///
/// In addition to the basic API, there are several useful pieces of
/// advanced functionality available on ``BezPath``:
///
/// -  ```flatten```_ does Bézier flattening, converting a curve to a series
///    of line segments
/// -  ```intersections```_ computes intersections of a path with a line,
///    useful for things like subdividing
///
/// .. _A Primer on Bézier Curves: https://pomax.github.io/bezierinfo/
/// .. _``intersections``: PathSeg::intersections
pub struct BezPath(pub KBezPath);

impl From<KBezPath> for BezPath {
    fn from(p: KBezPath) -> Self {
        Self(p)
    }
}
#[pymethods]
impl BezPath {
    #[new]
    fn __new__() -> Self {
        BezPath(KBezPath::new())
    }
    /// Push a "move to" element onto the path.
    #[pyo3(text_signature = "($self, pt)")]
    fn move_to(&mut self, p: Point) {
        self.0.move_to(p.0)
    }
    /// Push a "line to" element onto the path.
    #[pyo3(text_signature = "($self, pt)")]
    fn line_to(&mut self, p: Point) {
        self.0.line_to(p.0)
    }
    /// Push a "quad to" element onto the path.
    #[pyo3(text_signature = "($self, pt1, pt2)")]
    fn quad_to(&mut self, p1: Point, p2: Point) {
        self.0.quad_to(p1.0, p2.0)
    }
    /// Push a "curve to" element onto the path.
    #[pyo3(text_signature = "($self, pt1, pt2, pt3)")]
    fn curve_to(&mut self, p1: Point, p2: Point, p3: Point) {
        self.0.curve_to(p1.0, p2.0, p3.0)
    }
    /// Push a "close path" element onto the path.
    fn close_path(&mut self) {
        self.0.close_path();
    }
    // iter
    // segments
    /// Flatten the path, returning a list of points.
    fn flatten(&mut self, tolerance: f64) -> Vec<Point> {
        let mut v = vec![];
        self.0.flatten(tolerance, |l| match l {
            PathEl::MoveTo(p) => v.push(p.into()),
            PathEl::LineTo(p) => v.push(p.into()),
            _ => {}
        });
        v
    }
    /// Returns `true` if the path contains no segments.
    fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
    // apply_affine
    /// Is this path finite?
    fn is_finite(&self) -> bool {
        self.0.is_finite()
    }
    /// Is this path NaN?
    fn is_nan(&self) -> bool {
        self.0.is_nan()
    }
    /// Convert the path to an SVG path string representation.
    ///
    /// The current implementation doesn't take any special care to produce a
    /// short string (reducing precision, using relative movement).
    fn to_svg(&self) -> String {
        self.0.to_svg()
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

    /// Computes the intersections with a line as a list of ``Point`` objects.
    ///
    /// Note that this method is not in original kurbo
    #[pyo3(text_signature = "($self, line)")]
    fn intersections(&self, line: &Line) -> Vec<Point> {
        // XXX Not in original kurbo
        let mut intersections: Vec<Point> = vec![];
        for seg in self.0.segments() {
            for intersect in seg.intersect_line(line.0) {
                intersections.push(line.0.eval(intersect.line_t).into())
            }
        }
        intersections
    }

    // This is not in released kurbo yet

    /// Computes the minimum distance between this ``BezPath`` and another.
    ///
    /// Note that this method is not in original kurbo
    #[pyo3(text_signature = "($self, other)")]
    fn min_distance(&self, other: &BezPath) -> f64 {
        // XXX Not in original kurbo
        let segs1 = self.0.segments();
        let mut best_pair: Option<(f64, kurbo::PathSeg, kurbo::PathSeg)> = None;
        for s1 in segs1 {
            let p1 = vec![
                s1.eval(0.0),
                s1.eval(0.25),
                s1.eval(0.5),
                s1.eval(0.75),
                s1.eval(1.0),
            ];
            for s2 in other.0.segments() {
                let p2 = vec![
                    s2.eval(0.0),
                    s2.eval(0.25),
                    s2.eval(0.5),
                    s2.eval(0.75),
                    s2.eval(1.0),
                ];
                let dist = p1
                    .iter()
                    .zip(p2.iter())
                    .map(|(a, b)| a.distance(*b))
                    .min_by(|a, b| a.partial_cmp(b).unwrap_or(Ordering::Less))
                    .unwrap();
                if let Some((best, _, _)) = best_pair {
                    if dist > best {
                        continue;
                    }
                }
                best_pair = Some((dist, s1, s2));
            }
        }
        if let Some((_, s1, s2)) = best_pair {
            let curve1 = match s1 {
                PathSeg::Line(_) => PathSeg::Cubic(CubicBez::new(
                    s1.eval(0.0),
                    s1.eval(1.0 / 3.0),
                    s1.eval(2.0 / 3.0),
                    s1.eval(1.0),
                )),
                _ => s1,
            };
            let curve2 = match s2 {
                PathSeg::Line(_) => PathSeg::Cubic(CubicBez::new(
                    s2.eval(0.0),
                    s2.eval(1.0 / 3.0),
                    s2.eval(2.0 / 3.0),
                    s2.eval(1.0),
                )),
                _ => s2,
            };
            curve1.min_dist(curve2, 0.05).distance
        } else {
            f64::MAX
        }
    }
}
