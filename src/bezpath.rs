use crate::affine::Affine;
use crate::cubicbez::CubicBez;
use crate::line::Line;
use crate::pathel::PathEl;
use crate::pathseg::PathSeg;
use crate::point::Point;
use crate::quadbez::QuadBez;
use crate::rect::Rect;
use core::cmp::Ordering;
use itertools::Itertools;
use kurbo::{
    Affine as KAffine, BezPath as KBezPath, CubicBez as KCubicBez, ParamCurve, PathEl as KPathEl,
    PathSeg as KPathSeg, Shape, Vec2,
};
use pyo3::prelude::*;
use std::borrow::BorrowMut;
use std::sync::{Arc, Mutex, MutexGuard};

#[pyclass(subclass, module = "kurbopy")]
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
pub struct BezPath {
    _path: Arc<Mutex<KBezPath>>,
}

impl From<KBezPath> for BezPath {
    fn from(p: KBezPath) -> Self {
        Self {
            _path: Arc::new(Mutex::new(p)),
        }
    }
}

impl BezPath {
    pub(crate) fn path_mut(&mut self) -> MutexGuard<KBezPath> {
        self._path.borrow_mut().lock().unwrap()
    }

    pub(crate) fn path(&self) -> MutexGuard<KBezPath> {
        self._path.lock().unwrap()
    }
}

#[pymethods]
impl BezPath {
    #[new]
    fn __new__() -> Self {
        BezPath {
            _path: Arc::new(Mutex::new(KBezPath::new())),
        }
    }

    /// Removes the last [`PathEl`] from the path and returns it, or `None` if the path is empty.
    pub fn pop(&mut self) -> Option<PathEl> {
        self.path_mut().pop().map(|p| p.into())
    }

    /// Push a generic path element onto the path.
    pub fn push(&mut self, el: PathEl) {
        self.path_mut().push(el.0);
    }

    /// Push a "move to" element onto the path.
    #[pyo3(text_signature = "($self, pt)")]
    fn move_to(&mut self, p: Point) {
        self.path_mut().move_to(p.0);
    }
    /// Push a "line to" element onto the path.
    #[pyo3(text_signature = "($self, pt)")]
    fn line_to(&mut self, p: Point) {
        self.path_mut().line_to(p.0)
    }
    /// Push a "quad to" element onto the path.
    #[pyo3(text_signature = "($self, pt1, pt2)")]
    fn quad_to(&mut self, p1: Point, p2: Point) {
        self.path_mut().quad_to(p1.0, p2.0)
    }
    /// Push a "curve to" element onto the path.
    #[pyo3(text_signature = "($self, pt1, pt2, pt3)")]
    fn curve_to(&mut self, p1: Point, p2: Point, p3: Point) {
        self.path_mut().curve_to(p1.0, p2.0, p3.0)
    }
    /// Push a "close path" element onto the path.
    fn close_path(&mut self) {
        self.path_mut().close_path();
    }

    /// Shorten the path, keeping the first `len`` elements.
    fn truncate(&mut self, len: usize) {
        self.path_mut().truncate(len);
    }

    /// Flatten the path, returning a list of points.
    fn flatten(&mut self, tolerance: f64) -> Vec<Point> {
        let mut v = vec![];
        self.path().flatten(tolerance, |l| match l {
            KPathEl::MoveTo(p) => v.push(p.into()),
            KPathEl::LineTo(p) => v.push(p.into()),
            _ => {}
        });
        v
    }

    /// Get the segment at the given element index.
    ///
    /// If you need to access all segments, [`segments`] provides a better
    /// API. This is intended for random access of specific elements, for clients
    /// that require this specifically.
    ///
    /// **note**: This returns the segment that ends at the provided element
    /// index. In effect this means it is *1-indexed*: since no segment ends at
    /// the first element (which is presumed to be a `MoveTo`) `get_seg(0)` will
    /// always return `None`.
    fn get_seg(&self, ix: usize) -> Option<PathSeg> {
        self.path().get_seg(ix).map(|p| p.into())
    }

    /// Returns `true` if the path contains no segments.
    fn is_empty(&self) -> bool {
        self.path().is_empty()
    }

    /// Apply an affine transform to the path.
    fn apply_affine(&mut self, affine: Affine) {
        self.path_mut().apply_affine(affine.0)
    }

    /// Is this path finite?
    fn is_finite(&self) -> bool {
        self.path().is_finite()
    }
    /// Is this path NaN?
    fn is_nan(&self) -> bool {
        self.path().is_nan()
    }

    /// Returns a rectangle that conservatively encloses the path.
    ///
    /// Unlike the `bounding_box` method, this uses control points directly
    /// rather than computing tight bounds for curve elements.
    pub fn control_box(&self) -> Rect {
        self.path().control_box().into()
    }

    /// Returns a new path with the winding direction of all subpaths reversed.
    pub fn reverse_subpaths(&self) -> BezPath {
        self.path().reverse_subpaths().into()
    }

    /// Convert the path to an SVG path string representation.
    ///
    /// The current implementation doesn't take any special care to produce a
    /// short string (reducing precision, using relative movement).
    fn to_svg(&self) -> String {
        self.path().to_svg()
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
        self.path().area()
    }

    /// Total length of perimeter.
    #[pyo3(text_signature = "($self, accuracy)")]
    fn perimeter(&self, accuracy: f64) -> f64 {
        self.path().perimeter(accuracy)
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
        self.path().winding(pt.0)
    }

    /// The smallest rectangle that encloses the shape.
    fn bounding_box(&self) -> Rect {
        kurbo::Shape::bounding_box(&*self.path()).into()
    }

    /// Returns `true` if the [`Point`] is inside this shape.
    ///
    /// This is only meaningful for closed shapes.
    fn contains(&self, pt: Point) -> bool {
        self.winding(pt) != 0
    }

    /// Computes the intersections with a line as a list of ``Point`` objects.
    ///
    /// Note that this method is not in original kurbo
    #[pyo3(text_signature = "($self, line)")]
    fn intersections(&self, line: &Line) -> Vec<Point> {
        // XXX Not in original kurbo
        let mut intersections: Vec<Point> = vec![];
        for seg in self.path().segments() {
            for intersect in seg.intersect_line(line.0) {
                intersections.push(line.0.eval(intersect.line_t).into())
            }
        }
        intersections
    }

    /// Computes the minimum distance between this ``BezPath`` and another.
    ///
    /// Note that this method is not in original kurbo
    #[pyo3(text_signature = "($self, other)")]
    fn min_distance(&self, other: &BezPath) -> f64 {
        // XXX Not in original kurbo
        let path = self.path();
        let segs1 = path.segments();
        let mut best_pair: Option<(f64, kurbo::PathSeg, kurbo::PathSeg)> = None;
        for s1 in segs1 {
            let p1 = [
                s1.eval(0.0),
                s1.eval(0.25),
                s1.eval(0.5),
                s1.eval(0.75),
                s1.eval(1.0),
            ];
            for s2 in other.path().segments() {
                let p2 = [
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
                KPathSeg::Line(_) => KPathSeg::Cubic(KCubicBez::new(
                    s1.eval(0.0),
                    s1.eval(1.0 / 3.0),
                    s1.eval(2.0 / 3.0),
                    s1.eval(1.0),
                )),
                _ => s1,
            };
            let curve2 = match s2 {
                KPathSeg::Line(_) => KPathSeg::Cubic(KCubicBez::new(
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

    /// Returns true if the two BezPaths intersect
    ///
    /// Note that this method is not in original kurbo
    #[pyo3(text_signature = "($self, other)")]
    fn intersects(&self, other: &BezPath) -> Vec<Point> {
        let b1 = &self.path();
        let b2 = &other.path();
        if b1.bounding_box().intersect(b2.bounding_box()).area() < f64::EPSILON {
            return vec![];
        }
        let mut rv = vec![];
        let mut pts1 = vec![];
        let mut pts2 = vec![];
        b1.flatten(0.1, |el| match el {
            KPathEl::MoveTo(a) => pts1.push(a),
            KPathEl::LineTo(a) => pts1.push(a),
            _ => {}
        });
        b2.flatten(0.1, |el| match el {
            KPathEl::MoveTo(a) => pts2.push(a),
            KPathEl::LineTo(a) => pts2.push(a),
            _ => {}
        });
        for (&la1, &la2) in pts1.iter().circular_tuple_windows() {
            for (&lb1, &lb2) in pts2.iter().circular_tuple_windows() {
                let seg1 = KPathSeg::Line(kurbo::Line::new(la1, la2));
                let seg2 = kurbo::Line::new(lb1, lb2);
                rv.extend(
                    seg1.intersect_line(seg2)
                        .iter()
                        .map(|x| Point(seg1.eval(x.line_t))),
                );
            }
        }
        rv
    }

    #[pyo3(text_signature = "($self, scale_factor)")]
    fn scale_path(&self, scale_factor: f64) -> BezPath {
        let c = self.path().bounding_box().center();
        let c_vec = Vec2::new(c.x, c.y);
        BezPath {
            _path: Arc::new(Mutex::new(
                KAffine::translate(c_vec)
                    * KAffine::scale(scale_factor)
                    * KAffine::translate(c_vec * -1.0)
                    * &*self.path(),
            )),
        }
    }

    fn segments(&self) -> SegmentIterator {
        SegmentIterator {
            items: Arc::new(Mutex::new(self.path().clone())),
            index: 0,
        }
    }
    fn elements(&self) -> ElementIterator {
        ElementIterator {
            items: Arc::new(Mutex::new(self.path().clone())),
            index: 0,
        }
    }
}

#[pyclass]
struct SegmentIterator {
    items: Arc<Mutex<KBezPath>>,
    index: usize,
}

#[pymethods]
impl SegmentIterator {
    fn __iter__(slf: PyRef<'_, Self>) -> PyRef<'_, Self> {
        slf
    }
    fn __next__(&mut self, py: Python) -> Option<PyObject> {
        let item = self.items.lock().unwrap().segments().nth(self.index);
        self.index += 1;
        match item {
            None => None,
            Some(KPathSeg::Line(l)) => Some(Line::from(l).into_py(py)),
            Some(KPathSeg::Quad(q)) => Some(QuadBez::from(q).into_py(py)),
            Some(KPathSeg::Cubic(c)) => Some(CubicBez::from(c).into_py(py)),
        }
    }

    fn __len__(&self) -> usize {
        self.items.lock().unwrap().segments().count()
    }

    fn __getitem__(&self, ix: usize) -> PathSeg {
        self.items
            .lock()
            .unwrap()
            .segments()
            .nth(ix)
            .unwrap()
            .into()
    }
}

#[pyclass]
struct ElementIterator {
    items: Arc<Mutex<KBezPath>>,
    index: usize,
}

#[pymethods]
impl ElementIterator {
    fn __iter__(slf: PyRef<'_, Self>) -> PyRef<'_, Self> {
        slf
    }
    fn __next__(&mut self, py: Python) -> Option<PyObject> {
        let path = self.items.lock().unwrap();
        let item = path.elements().get(self.index);
        self.index += 1;
        item.map(|p| PathEl(*p).into_py(py))
    }

    fn __len__(&self) -> usize {
        self.items.lock().unwrap().elements().len()
    }
}
