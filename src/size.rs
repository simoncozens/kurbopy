use crate::rect::Rect;
use crate::vec2::Vec2;
use pyo3::types::PyType;

use kurbo::Size as KSize;
use pyo3::prelude::*;

#[pyclass(subclass, module = "kurbopy")]
#[derive(Clone, Debug)]
/// A 2D size.
pub struct Size(pub KSize);

impl From<KSize> for Size {
    fn from(p: KSize) -> Self {
        Self(p)
    }
}
#[pymethods]
impl Size {
    /// Create a new `Size` with the provided `width` and `height`.
    #[new]
    fn __new__(width: f64, height: f64) -> Size {
        Size(KSize::new(width, height))
    }

    // getters and setters
    #[getter]
    fn width(&self) -> f64 {
        self.0.width
    }
    #[setter]
    fn set_width(&mut self, width: f64) {
        self.0.width = width
    }
    #[getter]
    fn height(&self) -> f64 {
        self.0.height
    }
    #[setter]
    fn set_height(&mut self, height: f64) {
        self.0.height = height
    }

    #[classmethod]
    #[allow(non_snake_case)]
    /// A size with zero width or height.
    fn ZERO(_cls: &Bound<'_, PyType>) -> Self {
        Self(KSize::ZERO)
    }

    /// Returns the max of `width` and `height`.
    ///
    /// # Examples
    ///
    /// ```
    /// size = Size(-10.5, 42.0)
    /// assert size.max_side() == 42.0
    /// ```
    pub fn max_side(&self) -> f64 {
        self.0.max_side()
    }

    /// Returns the min of `width` and `height`.
    ///
    /// # Examples
    ///
    /// ```
    /// size = Size(-10.5, 42.0)
    /// assert size.min_side() == -10.5
    /// ```
    pub fn min_side(&self) -> f64 {
        self.0.min_side()
    }

    /// The area covered by this size.
    fn area(&self) -> f64 {
        self.0.area()
    }

    /// Whether this size has zero area.
    ///
    /// Note: a size with negative area is not considered empty.
    fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    /// Returns a new size bounded by `min` and `max.`
    ///
    /// # Examples
    ///
    /// ```
    /// this = Size(0., 100.)
    /// min = Size(10., 10.)
    /// max = Size(50., 50.)
    /// assert this.clamp(min, max) == Size(10., 50.))
    /// ```
    fn clamp(&self, min: Size, max: Size) -> Size {
        self.0.clamp(min.0, max.0).into()
    }

    /// Convert this size into a [`Vec2`], with `width` mapped to `x` and `height`
    /// mapped to `y`.
    fn to_vec2(&self) -> Vec2 {
        self.0.to_vec2().into()
    }

    /// Returns a new `Size`,
    /// with `width` and `height` rounded to the nearest integer.
    ///
    /// # Examples
    ///
    /// ```
    /// size_pos = Size(3.3, 3.6).round()
    /// assert size_pos.width == 3.0)
    /// assert size_pos.height == 4.0)
    /// size_neg = Size(-3.3, -3.6).round()
    /// assert size_neg.width == -3.0)
    /// assert size_neg.height == -4.0)
    /// ```
    fn round(&self) -> Size {
        self.0.round().into()
    }

    /// Returns a new `Size`,
    /// with `width` and `height` rounded up to the nearest integer,
    /// unless they are already an integer.
    fn ceil(&self) -> Size {
        self.0.ceil().into()
    }

    /// Returns a new `Size`,
    /// with `width` and `height` rounded down to the nearest integer,
    /// unless they are already an integer.
    fn floor(&self) -> Size {
        self.0.floor().into()
    }

    /// Returns a new `Size`,
    /// with `width` and `height` rounded away from zero to the nearest integer,
    /// unless they are already an integer.
    fn expand(&self) -> Size {
        self.0.expand().into()
    }

    /// Returns a new `Size`,
    /// with `width` and `height` rounded down towards zero the nearest integer,
    /// unless they are already an integer.
    fn trunc(&self) -> Size {
        self.0.trunc().into()
    }

    /// Returns the aspect ratio of a rectangle with the given size.
    ///
    /// If the width is `0`, the output will be `sign(self.height) * infinity`. If The width and
    /// height are `0`, then the output will be `NaN`.
    fn aspect_ratio(&self) -> f64 {
        self.0.aspect_ratio()
    }

    /// Convert this `Size` into a [`Rect`] with origin `(0.0, 0.0)`.
    pub fn to_rect(&self) -> Rect {
        self.0.to_rect().into()
    }

    /// Is this size finite?
    pub fn is_finite(&self) -> bool {
        self.0.is_finite()
    }
    /// Is this size NaN?
    pub fn is_nan(&self) -> bool {
        self.0.is_nan()
    }

    fn __imul__(&mut self, other: f64) {
        self.0 *= other;
    }
    fn __mul__(&self, other: f64) -> Size {
        (self.0 * other).into()
    }
    fn __rmul__(&self, other: f64) -> Size {
        (other * self.0).into()
    }
    fn __itruediv__(&mut self, other: f64) {
        self.0 /= other;
    }
    fn __truediv__(&self, other: f64) -> Size {
        (self.0 / other).into()
    }
    fn __add__(&self, other: Size) -> Size {
        (self.0 + other.0).into()
    }
    fn __iadd__(&mut self, other: Size) {
        self.0 += other.0;
    }
    fn __sub__(&self, other: Size) -> Size {
        (self.0 - other.0).into()
    }
    fn __isub__(&mut self, other: Size) {
        self.0 -= other.0;
    }
}
