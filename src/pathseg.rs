use crate::cubicbez::CubicBez;
use crate::line::Line;
use crate::point::Point;
use crate::quadbez::QuadBez;
use crate::rect::Rect;
use kurbo::PathSeg as KPathSeg;
use kurbo::{PathEl, Shape};
use pyo3::prelude::*;

#[pyclass(subclass)]
#[derive(Clone, Debug)]
pub struct PathSeg(pub KPathSeg);

impl From<KPathSeg> for PathSeg {
    fn from(p: KPathSeg) -> Self {
        Self(p)
    }
}

#[pymethods]
impl PathSeg {
    fn as_line(&self) -> Option<Line> {
        if let KPathSeg::Line(l) = self.0 {
            Some(l.into())
        } else {
            None
        }
    }

    fn as_quad(&self) -> Option<QuadBez> {
        if let KPathSeg::Quad(l) = self.0 {
            Some(l.into())
        } else {
            None
        }
    }

    fn as_cubic(&self) -> Option<CubicBez> {
        if let KPathSeg::Cubic(l) = self.0 {
            Some(l.into())
        } else {
            None
        }
    }
}
