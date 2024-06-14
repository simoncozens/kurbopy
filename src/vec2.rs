use crate::point::Point;
use kurbo::Vec2 as KVec2;
use pyo3::prelude::*;
use pyo3::types::PyType;

#[pyclass(subclass, module = "kurbopy")]
#[derive(Clone, Debug)]
/// A 2D vector.
///
/// This is intended primarily for a vector in the mathematical sense,
/// but it can be interpreted as a translation, and converted to and
/// from a point (vector relative to the origin) and size.
pub struct Vec2(pub KVec2);

impl From<KVec2> for Vec2 {
    fn from(p: KVec2) -> Self {
        Self(p)
    }
}

impl From<Vec2> for KVec2 {
    fn from(p: Vec2) -> Self {
        p.0
    }
}

#[pymethods]
impl Vec2 {
    #[new]
    fn __new__(x: f64, y: f64) -> Self {
        Vec2(KVec2::new(x, y))
    }

    #[classmethod]
    #[allow(non_snake_case)]
    /// The vector (0, 0).
    fn ZERO(_cls: &Bound<'_, PyType>) -> Self {
        Vec2(KVec2::ZERO)
    }

    /// Convert this vector into a :py:class:`Point`.
    fn to_point(&self) -> Point {
        self.0.to_point().into()
    }

    /// Dot product of two vectors.
    #[pyo3(text_signature = "($self, other)")]
    fn dot(&self, other: Vec2) -> f64 {
        self.0.dot(other.0)
    }

    /// Cross product of two vectors.
    ///
    /// This is signed so that (0, 1) × (1, 0) = 1.
    #[pyo3(text_signature = "($self, other)")]
    fn cross(&self, other: Vec2) -> f64 {
        self.0.cross(other.0)
    }

    /// Magnitude of vector.
    fn hypot(&self) -> f64 {
        self.0.hypot()
    }

    /// Magnitude squared of vector.
    fn hypot2(&self) -> f64 {
        self.0.hypot2()
    }

    /// Angle of vector.
    ///
    /// If the vector is interpreted as a complex number, this is the argument.
    /// The angle is expressed in radians.
    fn atan2(&self) -> f64 {
        self.0.atan2()
    }

    /// A unit vector of the given angle.
    ///
    /// With `th` at zero, the result is the positive X unit vector, and
    /// at π/2, it is the positive Y unit vector. The angle is expressed
    /// in radians.
    ///
    /// Thus, in a Y-down coordinate system (as is common for graphics),
    /// it is a clockwise rotation, and in Y-up (traditional for math), it
    /// is anti-clockwise. This convention is consistent with
    /// _`Affine.rotate`.
    #[classmethod]
    #[pyo3(text_signature = "(cls, th)")]
    fn from_angle(_cls: &Bound<'_, PyType>, th: f64) -> Self {
        KVec2::from_angle(th).into()
    }

    /// Linearly interpolate between two vectors.
    #[pyo3(text_signature = "($self, other, t)")]
    fn lerp(&self, other: Self, t: f64) -> Self {
        self.0.lerp(other.0, t).into()
    }

    /// Returns a vector of magnitude 1.0 with the same angle as `self`; i.e.
    /// a unit/direction vector.
    ///
    /// This produces `NaN` values when the magnitutde is `0`.
    fn normalize(&self) -> Self {
        self.0.normalize().into()
    }

    /// Returns a new `Vec2`,
    /// with `x` and `y` rounded to the nearest integer.
    fn round(&self) -> Self {
        self.0.round().into()
    }

    /// Returns a new `Vec2`,
    /// with `x` and `y` rounded up to the nearest integer,
    /// unless they are already an integer.
    fn ceil(&self) -> Self {
        self.0.ceil().into()
    }

    /// Returns a new `Vec2`,
    /// with `x` and `y` rounded down to the nearest integer,
    /// unless they are already an integer.
    fn floor(&self) -> Self {
        self.0.floor().into()
    }

    /// Returns a new `Vec2`,
    /// with `x` and `y` rounded away from zero to the nearest integer,
    /// unless they are already an integer.
    fn expand(&self) -> Self {
        self.0.expand().into()
    }

    /// Returns a new `Vec2`,
    /// with `x` and `y` rounded towards zero to the nearest integer,
    /// unless they are already an integer.
    fn trunc(&self) -> Self {
        self.0.trunc().into()
    }

    /// Is this Vec2 finite?
    fn is_finite(&self) -> bool {
        self.0.is_finite()
    }

    /// Is this Vec2 NaN?
    fn is_nan(&self) -> bool {
        self.0.is_nan()
    }

    #[getter]
    fn get_x(&self) -> f64 {
        self.0.x
    }
    #[getter]
    fn get_y(&self) -> f64 {
        self.0.y
    }
    #[setter]
    fn set_x(&mut self, x: f64) {
        self.0.x = x;
    }
    #[setter]
    fn set_y(&mut self, y: f64) {
        self.0.y = y;
    }

    fn __add__(&self, rhs: Self) -> PyResult<Self> {
        Ok((self.0 + rhs.0).into())
    }

    fn __iadd__(&mut self, other: Self) -> PyResult<()> {
        self.0 += other.0;
        Ok(())
    }

    fn __truediv__(&self, rhs: f64) -> PyResult<Self> {
        Ok((self.0 / rhs).into())
    }

    fn __itruediv__(&mut self, rhs: f64) -> PyResult<()> {
        self.0 /= rhs;
        Ok(())
    }

    fn __mul__(&self, rhs: f64) -> PyResult<Self> {
        Ok((self.0 * rhs).into())
    }

    fn __imul__(&mut self, rhs: f64) -> PyResult<()> {
        self.0 *= rhs;
        Ok(())
    }

    fn __neg__(&self) -> PyResult<Self> {
        Ok((-self.0).into())
    }

    fn __sub__(&self, rhs: Self) -> PyResult<Self> {
        Ok((self.0 - rhs.0).into())
    }

    fn __isub__(&mut self, other: Self) -> PyResult<()> {
        self.0 -= other.0;
        Ok(())
    }
}
