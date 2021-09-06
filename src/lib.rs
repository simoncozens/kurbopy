mod bezpath;
mod cubicbez;
mod line;
mod pathseg;
mod point;
mod quadbez;
mod rect;
mod translatescale;
mod vec2;

use pyo3::prelude::*;

#[pymodule]
fn kurbopy(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<bezpath::BezPath>()?;
    m.add_class::<cubicbez::CubicBez>()?;
    m.add_class::<line::Line>()?;
    m.add_class::<pathseg::PathSeg>()?;
    m.add_class::<point::Point>()?;
    m.add_class::<quadbez::QuadBez>()?;
    m.add_class::<rect::Rect>()?;
    m.add_class::<translatescale::TranslateScale>()?;
    m.add_class::<vec2::Vec2>()?;
    Ok(())
}
