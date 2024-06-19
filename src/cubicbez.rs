use crate::nearest::Nearest;
use crate::point::Point;
use crate::quadbez::QuadBez;
use crate::{
    impl_isfinitenan, impl_paramcurve, impl_paramcurvearclen, impl_paramcurvearea,
    impl_paramcurvecurvature, impl_paramcurvederiv, impl_paramcurveextrema, impl_paramcurvenearest,
    impl_shape_no_bounding_box,
};

use kurbo::{
    CubicBez as KCubicBez, ParamCurve, ParamCurveArclen, ParamCurveArea, ParamCurveCurvature,
    ParamCurveDeriv, ParamCurveExtrema, ParamCurveNearest,
};
use pyo3::prelude::*;

#[derive(Clone, Debug)]
#[pyclass(subclass, module = "kurbopy")]
/// A single cubic Bézier segment.
pub struct CubicBez(pub KCubicBez);

impl From<KCubicBez> for CubicBez {
    fn from(p: KCubicBez) -> Self {
        Self(p)
    }
}
#[pymethods]
impl CubicBez {
    #[new]
    fn __new__(p0: Point, p1: Point, p2: Point, p3: Point) -> Self {
        CubicBez(KCubicBez::new(p0.0, p1.0, p2.0, p3.0))
    }

    /// Convert to quadratic Béziers.
    ///
    /// Returns a list of tuples containing the start and end parameter in the cubic of each quadratic
    /// segment, along with the quadratic.
    ///
    /// Note that the resulting quadratic Béziers are not in general G1 continuous;
    /// they are optimized for minimizing distance error.
    ///
    /// This iterator will always produce at least one :py:class:`QuadBez`.
    #[inline]
    fn to_quads(&self, accuracy: f64) -> Vec<(f64, f64, QuadBez)> {
        self.0
            .to_quads(accuracy)
            .map(|(a, b, c)| (a, b, c.into()))
            .collect()
    }
    /// Determine the inflection points.
    ///
    /// Return value is t parameter for the inflection points of the curve segment.
    /// There are a maximum of two for a cubic Bézier.
    ///
    /// See <https://www.caffeineowl.com/graphics/2d/vectorial/cubic-inflexion.html>
    /// for the theory.
    pub fn inflections(&self) -> Vec<f64> {
        self.0.inflections().to_vec()
    }

    #[getter]
    fn get_p0(&self) -> Point {
        self.0.p0.into()
    }
    #[getter]
    fn get_p1(&self) -> Point {
        self.0.p1.into()
    }
    #[getter]
    fn get_p2(&self) -> Point {
        self.0.p2.into()
    }
    #[getter]
    fn get_p3(&self) -> Point {
        self.0.p3.into()
    }
    #[setter]
    fn set_p0(&mut self, p0: Point) {
        self.0.p0 = p0.0;
    }
    #[setter]
    fn set_p1(&mut self, p1: Point) {
        self.0.p1 = p1.0;
    }
    #[setter]
    fn set_p2(&mut self, p2: Point) {
        self.0.p2 = p2.0;
    }
    #[setter]
    fn set_p3(&mut self, p3: Point) {
        self.0.p3 = p3.0;
    }
}
impl_isfinitenan!(CubicBez);
impl_paramcurve!(CubicBez);
impl_paramcurvearclen!(CubicBez);
impl_paramcurvearea!(CubicBez);
impl_paramcurvecurvature!(CubicBez);
impl_paramcurvederiv!(CubicBez, QuadBez);
impl_paramcurveextrema!(CubicBez);
impl_paramcurvenearest!(CubicBez);
impl_shape_no_bounding_box!(CubicBez);
