use crate::insets::Insets;
use crate::point::Point;
use crate::size::Size;
use crate::vec2::Vec2;
use crate::{impl_isfinitenan, impl_shape, polymorphic};
use pyo3::types::PyType;

use kurbo::{Rect as KRect, Shape};
use pyo3::prelude::*;

#[pyclass(subclass, module = "kurbopy")]
#[derive(Clone, Debug)]
/// A rectangle.
pub struct Rect(pub KRect);

impl From<KRect> for Rect {
    fn from(p: KRect) -> Self {
        Self(p)
    }
}
#[pymethods]
impl Rect {
    #[new]
    fn __new__(p0: f64, p1: f64, p2: f64, p3: f64) -> Rect {
        Rect(KRect::new(p0, p1, p2, p3))
    }

    // getters and setters
    #[getter]
    fn x0(&self) -> f64 {
        self.0.x0
    }
    #[setter]
    fn set_x0(&mut self, x0: f64) {
        self.0.x0 = x0
    }
    #[getter]
    fn y0(&self) -> f64 {
        self.0.y0
    }
    #[setter]
    fn set_y0(&mut self, y0: f64) {
        self.0.y0 = y0
    }
    #[getter]
    fn x1(&self) -> f64 {
        self.0.x1
    }
    #[setter]
    fn set_x1(&mut self, x1: f64) {
        self.0.x1 = x1
    }

    #[classmethod]
    /// A new rectangle from two points.
    ///
    /// The result will have non-negative width and height.
    #[pyo3(text_signature = "(cls, p0, p1)")]
    fn from_points(_cls: &Bound<'_, PyType>, p0: Point, p1: Point) -> Rect {
        Rect(KRect::from_points(p0.0, p1.0))
    }

    #[classmethod]
    fn from_origin_size(_cls: &Bound<'_, PyType>, p0: Point, p1: Size) -> Self {
        Rect(KRect::from_origin_size(p0.0, p1.0))
    }

    #[classmethod]
    fn from_center_size(_cls: &Bound<'_, PyType>, p0: Point, p1: Size) -> Self {
        Rect(KRect::from_center_size(p0.0, p1.0))
    }

    /// Create a new `Rect` with the same size as `self` and a new origin.
    #[pyo3(text_signature = "($self, origin)")]
    fn with_origin(&self, origin: Point) -> Self {
        self.0.with_origin(origin.0).into()
    }

    fn with_size(&self, size: Size) -> Self {
        self.0.with_size(size.0).into()
    }

    fn inset(&self, inset: Insets) -> Self {
        self.0.inset(inset.0).into()
    }

    /// The width of the rectangle.
    ///
    /// Note: nothing forbids negative width.
    fn width(&self) -> f64 {
        self.0.width()
    }

    /// The height of the rectangle.
    ///
    /// Note: nothing forbids negative height.
    fn height(&self) -> f64 {
        self.0.height()
    }

    /// Returns the minimum value for the x-coordinate of the rectangle.
    fn min_x(&self) -> f64 {
        self.0.min_x()
    }
    /// Returns the maximum value for the x-coordinate of the rectangle.
    fn max_x(&self) -> f64 {
        self.0.max_x()
    }
    /// Returns the minimum value for the y-coordinate of the rectangle.
    fn min_y(&self) -> f64 {
        self.0.min_y()
    }
    /// Returns the maximum value for the y-coordinate of the rectangle.
    fn max_y(&self) -> f64 {
        self.0.max_y()
    }
    /// The origin of the rectangle.
    ///
    /// This is the top left corner in a y-down space and with
    /// non-negative width and height.
    fn origin(&self) -> Point {
        self.0.origin().into()
    }

    /// The size of the rectangle.
    fn size(&self) -> Size {
        self.0.size().into()
    }

    /// Whether this rectangle has zero area.
    ///
    /// Note: a rectangle with negative area is not considered empty.
    fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    /// The center point of the rectangle.
    fn center(&self) -> Point {
        self.0.center().into()
    }

    /// Take absolute value of width and height.
    ///
    /// The resulting rect has the same extents as the original, but is
    /// guaranteed to have non-negative width and height.
    fn abs(&self) -> Rect {
        self.0.abs().into()
    }

    /// The smallest rectangle enclosing two rectangles.
    ///
    /// Results are valid only if width and height are non-negative.
    fn union(&self, other: Rect) -> Rect {
        self.0.union(other.0).into()
    }

    /// Compute the union with one point.
    ///
    /// This method includes the perimeter of zero-area rectangles.
    /// Thus, a succession of ``union_pt`` operations on a series of
    /// points yields their enclosing rectangle.
    ///
    /// Results are valid only if width and height are non-negative.
    fn union_pt(&self, pt: Point) -> Rect {
        self.0.union_pt(pt.0).into()
    }

    /// The intersection of two rectangles.
    ///
    /// The result is zero-area if either input has negative width or
    /// height. The result always has non-negative width and height.
    fn intersect(&self, other: Rect) -> Rect {
        self.0.intersect(other.0).into()
    }

    /// Expand a rectangle by a constant amount in both directions.
    ///
    /// The logic simply applies the amount in each direction. If rectangle
    /// area or added dimensions are negative, this could give odd results.
    #[pyo3(text_signature = "($self, width, height)")]
    fn inflate(&self, width: f64, height: f64) -> Rect {
        self.0.inflate(width, height).into()
    }

    /// Returns a new `Rect`,
    /// with each coordinate value rounded to the nearest integer.
    fn round(&self) -> Rect {
        self.0.round().into()
    }
    /// Returns a new `Rect`,
    /// with each coordinate value rounded up to the nearest integer,
    /// unless they are already an integer.
    fn ceil(&self) -> Rect {
        self.0.ceil().into()
    }
    /// Returns a new `Rect`,
    /// with each coordinate value rounded down to the nearest integer,
    /// unless they are already an integer.
    fn floor(&self) -> Rect {
        self.0.floor().into()
    }
    /// Returns a new `Rect`,
    /// with each coordinate value rounded away from the center of the `Rect`
    /// to the nearest integer, unless they are already an integer.
    /// That is to say this function will return the smallest possible `Rect`
    /// with integer coordinates that is a superset of `self`.
    ///
    fn expand(&self) -> Rect {
        self.0.expand().into()
    }
    /// Returns a new `Rect`,
    /// with each coordinate value rounded towards the center of the `Rect`
    /// to the nearest integer, unless they are already an integer.
    /// That is to say this function will return the biggest possible `Rect`
    /// with integer coordinates that is a subset of `self`.
    fn trunc(&self) -> Rect {
        self.0.trunc().into()
    }
    /// Scales the ``Rect`` by ``factor`` with respect to the origin (the point ``(0, 0)``).
    ///
    /// Examples::
    ///
    ///     from kurbopy import Rect
    ///     rect = Rect(2, 2, 4, 6).scale_from_origin(2)
    ///     assert rect.x0 == 4
    ///     assert rect.x1 == 8
    #[pyo3(text_signature = "($self, factor)")]
    fn scale_from_origin(&self, factor: f64) -> Rect {
        self.0.scale_from_origin(factor).into()
    }

    /// The aspect ratio of the ``Rect``.
    ///
    /// This is defined as the height divided by the width. It measures the
    /// "squareness" of the rectangle (a value of `1` is square).
    ///
    /// If the width is `0` the output will be ``sign(y1 - y0) * infinity``.
    ///
    /// If the width and height are `0`, the result will be `NaN`.
    fn aspect_ratio(&self) -> f64 {
        self.0.aspect_ratio()
    }

    /// Returns the largest possible ``Rect`` that is fully contained in ``self``
    /// with the given ``aspect_ratio``.
    ///
    /// The aspect ratio is specified fractionally, as ``height / width``.
    ///
    /// The resulting rectangle will be centered if it is smaller than the
    /// input rectangle.
    ///
    /// For the special case where the aspect ratio is ``1.0``, the resulting
    /// ``Rect`` will be square.
    #[pyo3(text_signature = "($self, aspect_ratio)")]
    fn contained_rect_with_aspect_ratio(&self, aspect_ratio: f64) -> Rect {
        self.0.contained_rect_with_aspect_ratio(aspect_ratio).into()
    }

    fn __repr__(&self) -> String {
        format!("{:?}", self.0)
    }
}

impl_isfinitenan!(Rect);
impl_shape!(Rect);
polymorphic!(add Rect => (_add_Vec2, Vec2, Rect),
                         (_add_Insets, Insets, Rect)
);
polymorphic!(sub Rect => (_sub_Vec2, Vec2, Rect),
                         (_sub_Insets, Insets, Rect),
                         (_sub_Rect, Rect, Insets)
);

#[cfg(test)]
mod tests {
    use kurbo::{Insets, Rect};
    // use super::*;

    #[test]
    fn test_kurbo_docs() {
        let rect = Rect::new(0., 0., 5., 11.);
        let insets = Insets::uniform_xy(1., 7.);

        let inset_rect = rect + insets;
        println!("{:?}", inset_rect);
        let insets2 = inset_rect - rect;
        println!("{:?}", insets2);

        assert_eq!(insets2.x0, insets.x0);
        assert_eq!(insets2.y1, insets.y1);
        assert_eq!(insets2.x_value(), insets.x_value());
        assert_eq!(insets2.y_value(), insets.y_value());
    }
}
