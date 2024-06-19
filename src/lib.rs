mod affine;
mod arc;
mod bezpath;
mod circle;
mod common;
mod constpoint;
mod cubicbez;
mod ellipse;
mod insets;
mod line;
mod magic;
mod nearest;
mod paramcurve;
mod pathel;
mod pathseg;
mod point;
mod quadbez;
mod rect;
mod shape;
mod size;
mod translatescale;
mod vec2;

use pyo3::prelude::*;

#[pymodule]
fn kurbopy(_py: Python, m: &Bound<'_, PyModule>) -> PyResult<()> {
    register_child_module(m)?;
    m.add_class::<affine::Affine>()?;
    m.add_class::<arc::Arc>()?;
    m.add_class::<bezpath::BezPath>()?;
    m.add_class::<cubicbez::CubicBez>()?;
    m.add_class::<circle::Circle>()?;
    m.add_class::<constpoint::ConstPoint>()?;
    m.add_class::<ellipse::Ellipse>()?;
    m.add_class::<insets::Insets>()?;
    m.add_class::<line::Line>()?;
    m.add_class::<nearest::Nearest>()?;
    m.add_class::<pathseg::PathSeg>()?;
    m.add_class::<pathseg::LineIntersection>()?;
    m.add_class::<pathel::PathEl>()?;
    m.add_class::<point::Point>()?;
    m.add_class::<quadbez::QuadBez>()?;
    m.add_class::<rect::Rect>()?;
    m.add_class::<size::Size>()?;
    m.add_class::<translatescale::TranslateScale>()?;
    m.add_class::<vec2::Vec2>()?;
    Ok(())
}

fn register_child_module(parent_module: &Bound<'_, PyModule>) -> PyResult<()> {
    let child_module = PyModule::new_bound(parent_module.py(), "common")?;
    child_module.add_function(wrap_pyfunction!(
        common::factor_quartic_inner,
        &child_module
    )?)?;
    child_module.add_function(wrap_pyfunction!(common::solve_cubic, &child_module)?)?;
    child_module.add_function(wrap_pyfunction!(common::solve_itp, &child_module)?)?;
    child_module.add_function(wrap_pyfunction!(common::solve_quadratic, &child_module)?)?;
    child_module.add_function(wrap_pyfunction!(common::solve_quartic, &child_module)?)?;
    parent_module.add_submodule(&child_module)?;
    Ok(())
}
