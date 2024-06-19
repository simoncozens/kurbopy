use crate::impl_shape;
use crate::point::Point;
use crate::rect::Rect;
use crate::vec2::Vec2;

use kurbo::{Arc as KArc, Point as KPoint, Shape};
use pyo3::prelude::*;

#[derive(Clone, Debug)]
#[pyclass(subclass, module = "kurbopy")]
/// A single arc segment.
pub struct Arc(pub KArc);

impl From<KArc> for Arc {
    fn from(p: KArc) -> Self {
        Self(p)
    }
}

#[pymethods]
impl Arc {
    /// Create a new `Arc`.
    #[new]
    pub fn __new__(
        center: Point,
        radii: Vec2,
        start_angle: f64,
        sweep_angle: f64,
        x_rotation: f64,
    ) -> Self {
        Self(KArc::new(
            center.0,
            radii.0,
            start_angle,
            sweep_angle,
            x_rotation,
        ))
    }

    #[getter]
    pub fn get_center(&self) -> Point {
        self.0.center.into()
    }
    #[setter]
    pub fn set_center(&mut self, center: Point) {
        self.0.center = center.0
    }
    #[getter]
    pub fn get_radii(&self) -> Vec2 {
        self.0.radii.into()
    }
    #[setter]
    pub fn set_radii(&mut self, radii: Vec2) {
        self.0.radii = radii.0
    }
    #[getter]
    pub fn get_start_angle(&self) -> f64 {
        self.0.start_angle
    }
    #[setter]
    pub fn set_start_angle(&mut self, start_angle: f64) {
        self.0.start_angle = start_angle
    }
    #[getter]
    pub fn get_sweep_angle(&self) -> f64 {
        self.0.sweep_angle
    }
    #[setter]
    pub fn set_sweep_angle(&mut self, sweep_angle: f64) {
        self.0.sweep_angle = sweep_angle
    }
    #[getter]
    pub fn get_x_rotation(&self) -> f64 {
        self.0.x_rotation
    }
    #[setter]
    pub fn set_x_rotation(&mut self, x_rotation: f64) {
        self.0.x_rotation = x_rotation
    }

    /// Converts an Arc into a series of cubic bezier segments.
    ///
    /// Closure will be invoked for each segment.
    pub fn to_cubic_beziers(&self, py: Python, tolerance: f64, fun: Py<PyAny>) {
        let callback = |s: KPoint, m: KPoint, e: KPoint| {
            let _ = fun
                .call1(py, (Point(s), Point(m), Point(e)))
                .map_err(|e| e.restore(py));
        };
        self.0.to_cubic_beziers(tolerance, callback)
    }
}

impl_shape!(Arc);
