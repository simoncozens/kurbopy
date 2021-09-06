use crate::vec2::Vec2;
use kurbo::Point as KPoint;
use pyo3::prelude::*;
use pyo3::PyObjectProtocol;

use pyo3::PyNumberProtocol;

#[pyclass(subclass)]
#[derive(Clone, Debug)]
/// A 2D point.
#[pyo3(text_signature = "(x, y)")]
pub struct Point(pub KPoint);

impl From<KPoint> for Point {
    fn from(p: KPoint) -> Self {
        Self(p)
    }
}

#[pymethods]
impl Point {
    #[new]
    /// Create a new `Point` with the provided `x` and `y` coordinates.
    fn __new__(x: f64, y: f64) -> Self {
        Point(KPoint::new(x, y))
    }

    /// Convert this point into a `Vec2`.
    fn to_vec2(&self) -> Vec2 {
        self.0.to_vec2().into()
    }

    /// Linearly interpolate between two points.
    #[text_signature = "($self, other, t)"]
    fn lerp(&self, other: Self, t: f64) -> Self {
        self.0.lerp(other.0, t).into()
    }

    /// Determine the midpoint of two points.
    #[text_signature = "($self, other)"]
    fn midpoint(&self, other: Self) -> Self {
        self.0.midpoint(other.0).into()
    }

    /// Euclidean distance.
    #[text_signature = "($self, other)"]
    fn distance(&self, other: Self) -> f64 {
        self.0.distance(other.0)
    }

    /// Returns a new `Point`,
    /// with `x` and `y` rounded to the nearest integer.
    ///
    /// Examples::
    ///
    ///     from kurbopy import Point
    ///     a = Point(3.3, 3.6).round()
    ///     b = Point(3.0, -3.1).round()
    ///     assert a.x == 3.0
    ///     assert a.y == 4.0
    ///     assert b.x == 3.0
    ///     assert b.y == -3.0
    fn round(&self) -> Self {
        self.0.round().into()
    }

    /// Returns a new `Point`,
    /// with `x` and `y` rounded up to the nearest integer,
    /// unless they are already an integer.
    ///
    /// Examples::
    ///
    ///     from kurbopy import Point
    ///     a = Point(3.3, 3.6).ceil()
    ///     b = Point(3.0, -3.1).ceil()
    ///     assert a.x == 4.0
    ///     assert a.y == 4.0
    ///     assert b.x == 3.0
    ///     assert b.y == -3.0
    fn ceil(&self) -> Self {
        self.0.ceil().into()
    }

    /// Returns a new `Point`,
    /// with `x` and `y` rounded down to the nearest integer,
    /// unless they are already an integer.
    ///
    /// Examples::
    ///
    ///     from kurbopy import Point
    ///     a = Point(3.3, 3.6).floor()
    ///     b = Point(3.0, -3.1).floor()
    ///     assert a.x == 3.0
    ///     assert a.y == 3.0
    ///     assert b.x == 3.0
    ///     assert b.y == -4.0
    fn floor(&self) -> Self {
        self.0.floor().into()
    }

    /// Returns a new `Point`,
    /// with `x` and `y` rounded away from zero to the nearest integer,
    /// unless they are already an integer.
    ///
    /// Examples::
    ///
    ///     from kurbopy import Point
    ///     a = Point(3.3, 3.6).expand()
    ///     b = Point(3.0, -3.1).expand()
    ///     assert a.x == 4.0
    ///     assert a.y == 4.0
    ///     assert b.x == 3.0
    ///     assert b.y == -4.0
    fn expand(&self) -> Self {
        self.0.expand().into()
    }

    /// Returns a new `Point`,
    /// with `x` and `y` rounded towards zero to the nearest integer,
    /// unless they are already an integer.
    ///
    /// Examples::
    ///
    ///     from kurbopy import Point
    ///     a = Point(3.3, 3.6).trunc()
    ///     b = Point(3.0, -3.1).trunc()
    ///     assert a.x == 3.0
    ///     assert a.y == 3.0
    ///     assert b.x == 3.0
    ///     assert b.y == -3.0
    fn trunc(&self) -> Self {
        self.0.trunc().into()
    }

    /// Is this point finite?
    fn is_finite(&self) -> bool {
        self.0.is_finite()
    }

    /// Is this point NaN?
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

    fn _add_Vec2(&self, rhs: Vec2) -> Point {
        (self.0 + rhs.0).into()
    }

    fn _iadd_Vec2(&mut self, other: Vec2) {
        self.0 += other.0;
    }

    fn _sub_Vec2(&self, rhs: Vec2) -> Point {
        (self.0 - rhs.0).into()
    }

    fn _isub_Vec2(&mut self, other: Vec2) {
        self.0 -= other.0;
    }
}

#[pyproto]
impl PyNumberProtocol<'_> for Point {
    fn __add__(lhs: Self, rhs: &PyAny) -> PyResult<PyObject> {
        Python::with_gil(|py| {
            let magic = PyModule::import(py, "kurbopy.magic")?;
            magic.getattr("magic_add")?.call1((lhs, rhs))?.extract()
        })
    }
    fn __sub__(lhs: Self, rhs: &PyAny) -> PyResult<PyObject> {
        Python::with_gil(|py| {
            let magic = PyModule::import(py, "kurbopy.magic")?;
            magic.getattr("magic_sub")?.call1((lhs, rhs))?.extract()
        })
    }
    // I can't work out how to do magic iadd/isub
}

#[pyproto]
impl PyObjectProtocol<'_> for Point {
    fn __repr__(&self) -> PyResult<String> {
        Ok(format!("<Point x={:?} y={:?}>", self.0.x, self.0.y))
    }
}
