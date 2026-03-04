use crate::{impl_isfinitenan, impl_paramcurve, impl_paramcurvearclen, impl_paramcurvederiv};
use kurbo::{ConstPoint as KConstPoint, ParamCurve, ParamCurveArclen, ParamCurveDeriv};
use pyo3::prelude::*;

#[derive(Clone, Debug)]
#[pyclass(from_py_object, module = "kurbopy")]
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
