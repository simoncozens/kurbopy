use crate::bezpath::BezPath;
use crate::line::Line;
use crate::point::Point;
use crate::rect::Rect;
use crate::vec2::Vec2;
use crate::cubicbez::CubicBez;
use crate::polymorphic;

use kurbo::TranslateScale as KTranslateScale;
use pyo3::prelude::*;
use pyo3::types::PyType;

#[pyclass(subclass, module = "kurbopy")]
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
    #[pyo3(text_signature = "(cls, scale)")]
    fn scale(_cls: &Bound<PyType>, scale: f64) -> Self {
        TranslateScale(KTranslateScale::scale(scale))
    }

    #[classmethod]
    /// Create a new transformation with translation only.
    #[pyo3(text_signature = "(cls, vec2)")]
    fn translate(_cls: &Bound<PyType>, t: Vec2) -> Self {
        TranslateScale(KTranslateScale::translate(t))
    }

    /// Decompose transformation into translation and scale.
    fn as_tuple(&self) -> (Vec2, f64) {
        let t = self.0;
        (t.translation.into(), t.scale)
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

    #[allow(non_snake_case)]
    fn __add__(&self, rhs: Vec2) -> Self {
        (self.0 + rhs.0).into()
    }

    // We need to define this one manually because we don't have a 
    // newtype wrapper for BezPath.
    #[allow(non_snake_case)]
    fn _mul_BezPath(&self, bez: BezPath) -> BezPath {
        (self.0 * &*bez.path()).into()
    }
}


polymorphic!(mul TranslateScale =>
    (_mul_Point, Point, Point),
    (_mul_TranslateScale, TranslateScale, TranslateScale),
    (_mul_Line, Line, Line),
    (_mul_Rect, Rect, Rect),
    (_mul_CubicBez, CubicBez, CubicBez)
);
