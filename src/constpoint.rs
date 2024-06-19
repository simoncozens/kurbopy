use crate::{impl_isfinitenan, impl_paramcurve, impl_paramcurvearclen, impl_paramcurvederiv};
use kurbo::{ParamCurveDeriv, ParamCurveArclen, ParamCurve};
use kurbo::ConstPoint as KConstPoint;
use pyo3::prelude::*;

#[derive(Clone, Debug)]
#[pyclass(subclass, module = "kurbopy")]
/// A single ConstPoint segment.
pub struct ConstPoint(pub KConstPoint);

impl From<KConstPoint> for ConstPoint {
    fn from(p: KConstPoint) -> Self {
        Self(p)
    }
}

impl_isfinitenan!(ConstPoint);
impl_paramcurve!(ConstPoint);
impl_paramcurvearclen!(ConstPoint);
impl_paramcurvederiv!(ConstPoint, ConstPoint);
