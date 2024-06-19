use crate::arc::Arc;
use crate::bezpath::BezPath;
use crate::circle::Circle;
use crate::cubicbez::CubicBez;
use crate::ellipse::Ellipse;
use crate::line::Line;
use crate::pathel::PathEl;
use crate::pathseg::PathSeg;
use crate::point::Point;
use crate::quadbez::QuadBez;
use crate::rect::Rect;
use crate::vec2::Vec2;
use kurbo::Affine as KAffine;
use std::ops::Mul;
use crate::{impl_isfinitenan, polymorphic};

use pyo3::prelude::*;
use pyo3::types::PyType;

/// A 2D affine transform.
#[pyclass(subclass, module = "kurbopy")]
#[derive(Clone, Debug)]
pub struct Affine(pub KAffine);

impl From<KAffine> for Affine {
    fn from(p: KAffine) -> Self {
        Self(p)
    }
}
#[pymethods]
impl Affine {
    #[classmethod]
    #[allow(non_snake_case)]
    /// The identity transform.
    fn IDENTITY(_cls: &Bound<'_, PyType>) -> Self {
        Affine(KAffine::IDENTITY)
    }

    #[classmethod]
    #[allow(non_snake_case)]
    /// A transform that is flipped on the y-axis. Useful for converting between
    /// y-up and y-down spaces.
    fn FLIP_Y(_cls: &Bound<'_, PyType>) -> Self {
        Affine(KAffine::FLIP_Y)
    }

    #[classmethod]
    #[allow(non_snake_case)]
    /// A transform that is flipped on the x-axis.
    fn FLIP_X(_cls: &Bound<'_, PyType>) -> Self {
        Affine(KAffine::FLIP_X)
    }

    /// Construct an affine transform from coefficients.
    ///
    /// If the coefficients are `(a, b, c, d, e, f)`, then the resulting
    /// transformation represents this augmented matrix:
    ///
    /// ```text
    /// | a c e |
    /// | b d f |
    /// | 0 0 1 |
    /// ```
    ///
    /// Note that this convention is transposed from PostScript and
    /// Direct2D, but is consistent with the
    /// [Wikipedia](https://en.wikipedia.org/wiki/Affine_transformation)
    /// formulation of affine transformation as augmented matrix. The
    /// idea is that `(A * B) * v == A * (B * v)`, where `*` is the
    /// [`Mul`](std::ops::Mul) trait.
    #[new]
    fn __new__(a: f64, b: f64, c: f64, d: f64, e: f64, f: f64) -> Self {
        Affine(KAffine::new([a, b, c, d, e, f]))
    }

    /// An affine transform representing uniform scaling.
    #[classmethod]
    pub const fn scale(_cls: &Bound<'_, PyType>, s: f64) -> Affine {
        Affine(KAffine::scale(s))
    }

    /// An affine transform representing non-uniform scaling
    /// with different scale values for x and y
    #[classmethod]
    pub fn scale_non_uniform(_cls: &Bound<'_, PyType>, sx: f64, sy: f64) -> Affine {
        Affine(KAffine::scale_non_uniform(sx, sy))
    }

    /// An affine transform representing rotation.
    ///
    /// The convention for rotation is that a positive angle rotates a
    /// positive X direction into positive Y. Thus, in a Y-down coordinate
    /// system (as is common for graphics), it is a clockwise rotation, and
    /// in Y-up (traditional for math), it is anti-clockwise.
    ///
    /// The angle, `th`, is expressed in radians.
    #[classmethod]
    pub fn rotate(_cls: &Bound<'_, PyType>, th: f64) -> Affine {
        Affine(KAffine::rotate(th))
    }

    /// An affine transform representing a rotation of `th` radians about `center`.
    ///
    /// See [`Affine::rotate()`] for more info.
    #[classmethod]
    pub fn rotate_about(_cls: &Bound<'_, PyType>, th: f64, center: &Point) -> Affine {
        Affine(KAffine::rotate_about(th, center.0))
    }

    /// An affine transform representing translation.
    #[classmethod]
    pub fn translate(_cls: &Bound<'_, PyType>, p: &Point) -> Affine {
        Affine(KAffine::translate(p.to_vec2().0))
    }

    /// An affine transformation representing a skew.
    ///
    /// The `skew_x` and `skew_y` parameters represent skew factors for the
    /// horizontal and vertical directions, respectively.
    ///
    /// This is commonly used to generate a faux oblique transform for
    /// font rendering. In this case, you can slant the glyph 20 degrees
    /// clockwise in the horizontal direction (assuming a Y-up coordinate
    /// system):
    ///
    /// ```
    /// let oblique_transform = kurbo::Affine::skew(20f64.to_radians().tan(), 0.0);
    /// ```
    #[classmethod]
    pub fn skew(_cls: &Bound<'_, PyType>, skew_x: f64, skew_y: f64) -> Affine {
        Affine(KAffine::skew(skew_x, skew_y))
    }

    /// Create an affine transform that represents reflection about the line `point + direction * t, t in (-infty, infty)`
    ///
    /// # Examples
    ///
    /// ```
    /// # use kurbo::{Point, Vec2, Affine};
    /// # fn assert_near(p0: Point, p1: Point) {
    /// #     assert!((p1 - p0).hypot() < 1e-9, "{p0:?} != {p1:?}");
    /// # }
    /// let point = Point::new(1., 0.);
    /// let vec = Vec2::new(1., 1.);
    /// let map = Affine::reflect(point, vec);
    /// assert_near(map * Point::new(1., 0.), Point::new(1., 0.));
    /// assert_near(map * Point::new(2., 1.), Point::new(2., 1.));
    /// assert_near(map * Point::new(2., 2.), Point::new(3., 1.));
    /// ```
    #[classmethod]
    pub fn reflect(_cls: &Bound<'_, PyType>, point: &Point, direction: &Vec2) -> Affine {
        Affine(KAffine::reflect(point.0, direction.0))
    }

    /// A rotation by `th` followed by `self`.
    ///
    /// Equivalent to `self * Affine::rotate(th)`
    pub fn pre_rotate(&self, th: f64) -> Self {
        Affine(self.0 * KAffine::rotate(th))
    }

    /// A rotation by `th` about `center` followed by `self`.
    ///
    /// Equivalent to `self * Affine::rotate_about(th)`
    fn pre_rotate_about(&self, th: f64, center: &Point) -> Self {
        Affine(self.0 * KAffine::rotate_about(th, center.0))
    }

    /// A scale by `scale` followed by `self`.
    ///
    /// Equivalent to `self * Affine::scale(scale)`
    pub fn pre_scale(&self, scale: f64) -> Self {
        Affine(self.0 * KAffine::scale(scale))
    }

    /// A scale by `(scale_x, scale_y)` followed by `self`.
    ///
    /// Equivalent to `self * Affine::scale_non_uniform(scale_x, scale_y)`
    pub fn pre_scale_non_uniform(&self, scale_x: f64, scale_y: f64) -> Self {
        Affine(self.0 * KAffine::scale_non_uniform(scale_x, scale_y))
    }

    /// A translation of `trans` followed by `self`.
    ///
    /// Equivalent to `self * Affine::translate(trans)`
    pub fn pre_translate(&self, trans: &Vec2) -> Self {
        Affine(self.0 * KAffine::translate(trans.0))
    }

    /// `self` followed by a rotation of `th`.
    ///
    /// Equivalent to `Affine::rotate(th) * self`
    pub fn then_rotate(&self, th: f64) -> Self {
        Affine(KAffine::rotate(th) * self.0)
    }

    /// `self` followed by a rotation of `th` about `center`.
    ///
    /// Equivalent to `Affine::rotate_about(th, center) * self`
    pub fn then_rotate_about(&self, th: f64, center: &Point) -> Self {
        Affine(KAffine::rotate_about(th, center.0) * self.0)
    }

    /// `self` followed by a scale of `scale`.
    ///
    /// Equivalent to `Affine::scale(scale) * self`
    pub fn then_scale(&self, scale: f64) -> Self {
        Affine(KAffine::scale(scale) * self.0)
    }

    /// `self` followed by a scale of `(scale_x, scale_y)`.
    ///
    /// Equivalent to `Affine::scale_non_uniform(scale_x, scale_y) * self`
    pub fn then_scale_non_uniform(&self, scale_x: f64, scale_y: f64) -> Self {
        Affine(KAffine::scale_non_uniform(scale_x, scale_y) * self.0)
    }

    /// `self` followed by a translation of `trans`.
    ///
    /// Equivalent to `Affine::translate(trans) * self`
    pub fn then_translate(&self, trans: &Vec2) -> Self {
        Affine(KAffine::translate(trans.0) * self.0)
    }

    /// Creates an affine transformation that takes the unit square to the given rectangle.
    ///
    /// Useful when you want to draw into the unit square but have your output fill any rectangle.
    /// In this case push the `Affine` onto the transform stack.
    #[classmethod]
    pub fn map_unit_square(_cls: &Bound<'_, PyType>, rect: &Rect) -> Affine {
        Affine(KAffine::map_unit_square(rect.0))
    }

    /// Get the coefficients of the transform.
    pub fn as_coeffs(&self) -> [f64; 6] {
        self.0.as_coeffs()
    }

    /// Compute the determinant of this transform.
    pub fn determinant(&self) -> f64 {
        self.0.determinant()
    }

    /// Compute the inverse transform.
    ///
    /// Produces NaN values when the determinant is zero.
    pub fn inverse(&self) -> Affine {
        Affine(self.0.inverse())
    }

    /// Compute the bounding box of a transformed rectangle.
    ///
    /// Returns the minimal `Rect` that encloses the given `Rect` after affine transformation.
    /// If the transform is axis-aligned, then this bounding box is "tight", in other words the
    /// returned `Rect` is the transformed rectangle.
    ///
    /// The returned rectangle always has non-negative width and height.
    pub fn transform_rect_bbox(&self, rect: &Rect) -> Rect {
        Rect(self.0.transform_rect_bbox(rect.0))
    }

    /// Returns the translation part of this affine map (`(self.0[4], self.0[5])`).
    pub fn translation(&self) -> Vec2 {
        Vec2(self.0.translation())
    }

    /// Replaces the translation portion of this affine map
    ///
    /// The translation can be seen as being applied after the linear part of the map.
    pub fn with_translation(&self, trans: &Vec2) -> Affine {
        Affine(self.0.with_translation(trans.0))
    }

    #[allow(non_snake_case)]
    fn _mul_BezPath(&self, rhs: &BezPath) -> BezPath {
        let path = rhs.path().clone();
        self.0.mul(path).into()
    }

    fn __eq__(&self, other: &Affine) -> bool {
        self.0 == other.0
    }

    #[allow(non_snake_case)]
    fn __rmul__(&self, rhs: f64) -> Affine {
        Affine(rhs * self.0)
    }
}

impl_isfinitenan!(Affine);
polymorphic!(mul Affine =>
    (_mul_Point, Point, Point),
    (_mul_Affine, Affine, Affine),
    (_mul_Arc, Arc, Arc),
    (_mul_Circle, Circle, Ellipse),
    (_mul_CubicBez, CubicBez, CubicBez),
    (_mul_Line, Line, Line),
    (_mul_PathEl, PathEl, PathEl),
    (_mul_PathSeg, PathSeg, PathSeg),
    (_mul_QuadBez, QuadBez, QuadBez),
    (_mul_Ellipse, Ellipse, Ellipse)
);