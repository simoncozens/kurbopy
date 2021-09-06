use crate::bezpath::BezPath;
use crate::line::Line;
use crate::point::Point;
use crate::vec2::Vec2;

use kurbo::TranslateScale as KTranslateScale;
use pyo3::prelude::*;
use pyo3::types::PyType;
use pyo3::PyNumberProtocol;

#[pyclass(subclass)]
#[derive(Clone, Debug)]
/// A transformation including scaling and translation.
///
/// If the translation is `(x, y)` and the scale is `s`, then this
/// transformation represents this augmented matrix::
///
///     | s 0 x |
///     | 0 s y |
///     | 0 0 1 |
///
/// See [`Affine`] for more details about the
/// equivalence with augmented matrices.
///
/// Various multiplication ops are defined, and these are all defined
/// to be consistent with matrix multiplication. Therefore,
/// ``TranslateScale * Point`` is defined but not the other way around.
///
/// Also note that multiplication is not commutative. Thus,
/// ``TranslateScale.scale(2.0) * TranslateScale.translate(Vec2(1.0, 0.0))``
/// has a translation of (2, 0), while
/// ``TranslateScale.translate(Vec2(1.0, 0.0)) * TranslateScale.scale(2.0)``
/// has a translation of (1, 0). (Both have a scale of 2).
///
/// This transformation is less powerful than `Affine`, but can be applied
/// to more primitives, especially including :py:class:`Rect`.
#[pyo3(text_signature = "(translation, scale)")]
pub struct TranslateScale(pub KTranslateScale);

impl From<KTranslateScale> for TranslateScale {
    fn from(p: KTranslateScale) -> Self {
        Self(p)
    }
}
#[pymethods]
impl TranslateScale {
    #[new]
    /// Create a new transformation from translation and scale.
    fn __new__(translation: Vec2, scale: f64) -> Self {
        TranslateScale(KTranslateScale::new(translation.into(), scale))
    }

    #[classmethod]
    /// Create a new transformation with scale only.
    #[text_signature = "(cls, scale)"]
    fn scale(_cls: &PyType, scale: f64) -> Self {
        TranslateScale(KTranslateScale::scale(scale))
    }

    #[classmethod]
    /// Create a new transformation with translation only.
    #[text_signature = "(cls, vec2)"]
    fn translate(_cls: &PyType, t: Vec2) -> Self {
        TranslateScale(KTranslateScale::translate(t.into()))
    }

    /// Decompose transformation into translation and scale.
    fn as_tuple(&self) -> (Vec2, f64) {
        let t = self.0.as_tuple();
        (t.0.into(), t.1)
    }

    /// Compute the inverse transform.
    ///
    /// Multiplying a transform with its inverse (either on the
    /// left or right) results in the identity transform
    /// (modulo floating point rounding errors).
    ///
    /// Produces NaN values when scale is zero.
    fn inverse(&self) -> Self {
        self.0.inverse().into()
    }

    /// Is this translate/scale finite?
    fn is_finite(&self) -> bool {
        self.0.is_finite()
    }

    /// Is this translate/scale NaN?
    fn is_nan(&self) -> bool {
        self.0.is_nan()
    }

    fn _add_Vec2(&self, rhs: Vec2) -> Self {
        (self.0 + rhs.0).into()
    }
    fn _mul_Point(&self, rhs: Point) -> Point {
        (self.0 * rhs.0).into()
    }
    fn _mul_TranslateScale(&self, rhs: TranslateScale) -> TranslateScale {
        (self.0 * rhs.0).into()
    }
    fn _mul_BezPath(&self, rhs: BezPath) -> BezPath {
        (self.0 * rhs.0).into()
    }
    fn _mul_Line(&self, rhs: Line) -> PyResult<Line> {
        let p: Line = (self.0 * rhs.0).into();
        Ok(p)
    }
}

#[pyproto]
impl PyNumberProtocol<'_> for TranslateScale {
    fn __mul__(lhs: Self, rhs: &PyAny) -> PyResult<PyObject> {
        Python::with_gil(|py| {
            let magic = PyModule::import(py, "kurbopy.magic")?;
            magic.getattr("magic_mul")?.call1((lhs, rhs))?.extract()
        })
    }
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
}
