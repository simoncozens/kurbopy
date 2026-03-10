use kurbo::ParamCurveFit;
use pyo3::prelude::*;
use pyo3::Bound;

struct PythonFit<'a>(&'a Bound<'a, PyAny>);

impl ParamCurveFit for PythonFit<'_> {
    fn sample_pt_tangent(&self, t: f64, sign: f64) -> kurbo::CurveFitSample {
        // Call the Python method `sample_pt_tangent` on the wrapped object.
        let result = self
            .0
            .call_method1("sample_pt_tangent", (t, sign))
            .expect("Failed to call sample_pt_tangent");
        // Expect the result to be a tuple of (Point, Vec2).
        let (pt, tangent): (crate::point::Point, crate::vec2::Vec2) =
            result.extract().expect("Expected a tuple of (Point, Vec2)");
        kurbo::CurveFitSample {
            p: pt.0,
            tangent: tangent.0,
        }
    }

    fn sample_pt_deriv(&self, t: f64) -> (kurbo::Point, kurbo::Vec2) {
        let result = self
            .0
            .call_method1("sample_pt_deriv", (t,))
            .expect("Failed to call sample_pt_deriv");
        let (pt, deriv): (crate::point::Point, crate::vec2::Vec2) =
            result.extract().expect("Expected a tuple of (Point, Vec2)");
        (pt.0, deriv.0)
    }

    fn break_cusp(&self, range: std::ops::Range<f64>) -> Option<f64> {
        let result = self
            .0
            .call_method1("break_cusp", (range.start, range.end))
            .expect("Failed to call break_cusp");
        result.extract().ok()
    }
}

#[pyfunction]
pub(crate) fn fit_to_bezpath(
    py: Python<'_>,
    obj: Py<PyAny>,
    accuracy: f64,
) -> crate::bezpath::BezPath {
    let fit = PythonFit(obj.bind(py));
    kurbo::fit_to_bezpath(&fit, accuracy).into()
}
