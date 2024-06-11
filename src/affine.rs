use kurbo::{Affine as KAffine, TranslateScale as KTranslateScale};

use pyo3::prelude::*;

/// A 2D affine transform.
#[pyclass(subclass)]
#[derive(Clone, Debug)]
pub struct Affine(pub KAffine);
