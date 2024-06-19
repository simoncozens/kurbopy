use crate::affine::Affine;
use crate::{impl_isfinitenan, impl_shape};
use crate::point::Point;
use crate::rect::Rect;
use crate::vec2::Vec2;

use kurbo::{Ellipse as KEllipse, Shape};
use pyo3::prelude::*;
use pyo3::types::PyType;

#[derive(Clone, Debug)]
#[pyclass(subclass, module = "kurbopy")]
/// A Ellipse.
pub struct Ellipse(pub KEllipse);

impl From<KEllipse> for Ellipse {
    fn from(p: KEllipse) -> Self {
        Self(p)
    }
}

#[pymethods]
impl Ellipse {
    /// Create a new `Ellipse`.
    #[new]
    pub fn __new__(center: Point, radii: Vec2, x_rotation: f64) -> Self {
        Self(KEllipse::new(center.0, radii.0, x_rotation))
    }
    #[getter]
    pub fn get_center(&self) -> Point {
        self.0.center().into()
    }
    #[getter]
    pub fn get_radii(&self) -> Vec2 {
        self.0.radii().into()
    }
    #[getter]
    pub fn get_rotation(&self) -> f64 {
        self.0.rotation()
    }
    /// Returns the radii and the rotation of this ellipse.
    ///
    /// Equivalent to `(self.radii(), self.rotation())` but more efficient.
    fn radii_and_rotation(&self) -> (Vec2, f64) {
        (self.0.radii().into(), self.0.rotation())
    }

    #[classmethod]
    /// Returns the largest ellipse that can be bounded by this [`Rect`].
    ///
    /// This uses the absolute width and height of the rectangle.
    ///
    /// This ellipse is always axis-aligned; to apply rotation you can call
    /// [`with_rotation`] with the result.
    ///
    /// [`with_rotation`]: Ellipse::with_rotation    #[pyo3(text_signature = "(cls, p0, p1)")]
    fn from_rect(_cls: &Bound<'_, PyType>, rect: Rect) -> Ellipse {
        Ellipse(KEllipse::from_rect(rect.0))
    }

    #[classmethod]
    /// Create an ellipse from an affine transformation of the unit circle.
    pub fn from_affine(_cls: &Bound<'_, PyType>, affine: Affine) -> Self {
        Ellipse(KEllipse::from_affine(affine.0))
    }

    /// Create a new `Ellipse` centered on the provided point.
    pub fn with_center(&self, new_center: Point) -> Ellipse {
        Ellipse(self.0.with_center(new_center.0))
    }

    /// Create a new `Ellipse` with the provided radii.
    pub fn with_radii(&self, new_radii: Vec2) -> Ellipse {
        Ellipse(self.0.with_radii(new_radii.0))
    }

    /// Create a new `Ellipse`, with the rotation replaced by `rotation`
    /// radians.
    ///
    /// The rotation is clockwise, for a y-down coordinate system. For more
    /// on rotation, See [`Affine::rotate`].
    fn with_rotation(&self, rotation: f64) -> Ellipse {
        Ellipse(self.0.with_rotation(rotation))
    }

    #[allow(non_snake_case)]
    fn __add__(&self, rhs: Vec2) -> Ellipse {
        Ellipse(self.0 + rhs.0)
    }
    #[allow(non_snake_case)]
    fn __sub__(&self, rhs: Vec2) -> Ellipse {
        Ellipse(self.0 + rhs.0)
    }
}

impl_isfinitenan!(Ellipse);
impl_shape!(Ellipse);
