use crate::point::Point;
use crate::rect::Rect;
use crate::vec2::Vec2;
use crate::{impl_isfinitenan, impl_shape};

use kurbo::{Circle as KCircle, CircleSegment as KCircleSegment, Shape};
use pyo3::prelude::*;

#[derive(Clone, Debug)]
#[pyclass(subclass, module = "kurbopy")]
/// A circle.
pub struct Circle(pub KCircle);

impl From<KCircle> for Circle {
    fn from(p: KCircle) -> Self {
        Self(p)
    }
}

#[pymethods]
impl Circle {
    /// Create a new `Circle`.
    #[new]
    pub fn __new__(center: Point, radius: f64) -> Self {
        Self(KCircle::new(center.0, radius))
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
    pub fn get_radius(&self) -> f64 {
        self.0.radius
    }
    #[setter]
    pub fn set_radius(&mut self, radius: f64) {
        self.0.radius = radius
    }

    /// Create a [`CircleSegment`] by cutting out parts of this circle.
    pub fn segment(&self, inner_radius: f64, start_angle: f64, sweep_angle: f64) -> CircleSegment {
        CircleSegment(self.0.segment(inner_radius, start_angle, sweep_angle))
    }

    fn __add__(&self, v: Vec2) -> Circle {
        Circle(self.0 + v.0)
    }
    fn __sub__(&self, v: Vec2) -> Circle {
        Circle(self.0 - v.0)
    }
}
impl_isfinitenan!(Circle);
impl_shape!(Circle);

#[derive(Clone, Debug)]
#[pyclass(subclass, module = "kurbopy")]
/// A segment of a circle.
///
/// If `inner_radius > 0`, then the shape will be a doughnut segment.
pub struct CircleSegment(pub KCircleSegment);

impl From<KCircleSegment> for CircleSegment {
    fn from(p: KCircleSegment) -> Self {
        Self(p)
    }
}

#[pymethods]
impl CircleSegment {
    /// Create a `CircleSegment` out of its constituent parts.
    #[new]
    pub fn __new__(
        center: Point,
        outer_radius: f64,
        inner_radius: f64,
        start_angle: f64,
        sweep_angle: f64,
    ) -> Self {
        Self(KCircleSegment::new(
            center.0,
            outer_radius,
            inner_radius,
            start_angle,
            sweep_angle,
        ))
    }

    // getters and setters
    #[getter]
    pub fn get_center(&self) -> Point {
        self.0.center.into()
    }
    #[setter]
    pub fn set_center(&mut self, center: Point) {
        self.0.center = center.0
    }
    #[getter]
    pub fn get_outer_radius(&self) -> f64 {
        self.0.outer_radius
    }
    #[setter]
    pub fn set_outer_radius(&mut self, outer_radius: f64) {
        self.0.outer_radius = outer_radius
    }
    #[getter]
    pub fn get_inner_radius(&self) -> f64 {
        self.0.inner_radius
    }
    #[setter]
    pub fn set_inner_radius(&mut self, inner_radius: f64) {
        self.0.inner_radius = inner_radius
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

    fn __add__(&self, v: Vec2) -> CircleSegment {
        CircleSegment(self.0 + v.0)
    }
    fn __sub__(&self, v: Vec2) -> CircleSegment {
        CircleSegment(self.0 - v.0)
    }
}
impl_isfinitenan!(CircleSegment);
impl_shape!(CircleSegment);
