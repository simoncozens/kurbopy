use crate::impl_isfinitenan;
use crate::rect::Rect;
use crate::size::Size;
use kurbo::Insets as KInsets;
use pyo3::prelude::*;
use pyo3::types::PyType;

#[derive(Clone, Debug)]
#[pyclass(subclass, module = "kurbopy")]
/// Insets from the edges of a rectangle.
///
///
/// The inset value for each edge can be thought of as a delta computed from
/// the center of the rect to that edge. For instance, with an inset of `2.0` on
/// the x-axis, a rectangle with the origin `(0.0, 0.0)` with that inset added
/// will have the new origin at `(-2.0, 0.0)`.
///
/// Put alternatively, a positive inset represents increased distance from center,
/// and a negative inset represents decreased distance from center.
///
/// ## Examples
///
/// Positive insets added to a [`Rect`] produce a larger [`Rect`]:
/// ```
/// rect = Rect.from_origin_size(Point(0., 0.,), Size(10., 10.,))
/// insets = Insets.uniform_xy(3., 0.)
///
/// inset_rect = rect + insets
/// assert inset_rect.width() == 16.0, "10.0 + 3.0 × 2"
/// assert inset_rect.x0 == -3.0
/// ```
///
/// Negative insets added to a [`Rect`] produce a smaller [`Rect`]:
///
/// ```
/// rect = Rect.from_origin_size(Point(0., 0.,), Size(10., 10.,))
/// insets = Insets.uniform_xy(-3., 0.)
///
/// inset_rect = rect + insets
/// assert inset_rect.width() == 4.0, "10.0 - 3.0 × 2"
/// assert inset_rect.x0 == 3.0
/// ```
///
/// [`Insets`] operate on the absolute rectangle [`Rect.abs`], and so ignore
/// existing negative widths and heights.
///
/// ```
/// rect = Rect(7., 11., 0., 0.)
/// insets = Insets.uniform_xy(0., 1.)
///
/// assert rect.width() == -7.0
///
/// inset_rect = rect + insets
/// assert inset_rect.width() == 7.0
/// assert inset_rect.x0 == 0.0
/// assert inset_rect.height() == 13.0
/// ```
///
/// The width and height of an inset operation can still be negative if the
/// [`Insets`]' dimensions are greater than the dimensions of the original [`Rect`].
///
/// ```
/// rect = Rect(Point(0., 0.), Point(3., 5.))
/// insets = Insets.uniform_xy(0., 7.)
///
/// inset_rect = rect - insets
/// assert inset_rect.height() == -9., "5 - 7 × 2"
/// ```
///
/// `Rect - Rect = Insets`:
///
///
/// ```
/// rect = Rect(Point(0., 0.), Point(5., 11.))
/// insets = Insets.uniform_xy(1., 7.,)
///
/// inset_rect = rect + insets
/// insets2 = inset_rect - rect
///
/// assert insets2.x0 == insets.x0
/// assert insets2.y1 == insets.y1
/// assert insets2.x_value() == insets.x_value()
/// assert insets2.y_value() == insets.y_value()
/// ```
pub struct Insets(pub KInsets);

impl From<KInsets> for Insets {
    fn from(p: KInsets) -> Self {
        Self(p)
    }
}

#[pymethods]
impl Insets {
    /// Create a new `Insets`.
    #[new]
    pub fn __new__(x0: f64, y0: f64, x1: f64, y1: f64) -> Self {
        Self(KInsets { x0, y0, x1, y1 })
    }
    // getters and setters
    #[getter]
    pub fn x0(&self) -> f64 {
        self.0.x0
    }
    #[setter]
    pub fn set_x0(&mut self, x0: f64) {
        self.0.x0 = x0
    }
    #[getter]
    pub fn y0(&self) -> f64 {
        self.0.y0
    }
    #[setter]
    pub fn set_y0(&mut self, y0: f64) {
        self.0.y0 = y0
    }
    #[getter]
    pub fn x1(&self) -> f64 {
        self.0.x1
    }
    #[setter]
    pub fn set_x1(&mut self, x1: f64) {
        self.0.x1 = x1
    }
    #[getter]
    pub fn y1(&self) -> f64 {
        self.0.y1
    }
    #[setter]
    pub fn set_y1(&mut self, y1: f64) {
        self.0.y1 = y1
    }
    
    #[classmethod]
    #[allow(non_snake_case)]
    /// Zeroed insets
    fn ZERO(_cls: &Bound<'_, PyType>) -> Self {
        Self(KInsets::ZERO)
    }

    /// New uniform insets.
    #[classmethod]
    fn uniform(_cls: &Bound<'_, PyType>, value: f64) -> Self {
        Self(KInsets::uniform(value))
    }
    /// New insets with uniform values along each axis.
    #[classmethod]
    pub fn uniform_xy(_cls: &Bound<'_, PyType>, x_value: f64, y_value: f64) -> Self {
        Self(KInsets::uniform_xy(x_value, y_value))
    }

    /// The total delta on the x-axis represented by these insets.
    ///
    /// # Examples
    ///
    /// ```
    /// insets = Insets.uniform_xy(3., 8.)
    /// assert insets.x_value() == 6
    ///
    /// insets = Insets(5., 0., -12., 0.,)
    /// assert insets.x_value() == -7
    /// ```
    pub fn x_value(&self) -> f64 {
        self.0.x_value()
    }

    /// The total delta on the y-axis represented by these insets.
    ///
    /// # Examples
    ///
    /// ```
    /// insets = Insets.uniform_xy(3., 7.)
    /// assert insets.y_value() == 14
    ///
    /// insets = Insets(5., 10., -12., 4.,)
    /// assert insets.y_value() == 14
    /// ```
    pub fn y_value(&self) -> f64 {
        self.0.y_value()
    }

    /// Returns the total delta represented by these insets as a [`Size`].
    ///
    /// This is equivalent to creating a [`Size`] from the values returned by
    /// [`x_value`] and [`y_value`].
    ///
    /// This function may return a size with negative values.
    fn size(&self) -> Size {
        self.0.size().into()
    }

    /// Return `true` iff all values are nonnegative.
    pub fn are_nonnegative(&self) -> bool {
        self.0.are_nonnegative()
    }

    /// Return new `Insets` with all negative values replaced with `0.0`.
    ///
    /// This is provided as a convenience for applications where negative insets
    /// are not meaningful.
    pub fn nonnegative(&self) -> Insets {
        self.0.nonnegative().into()
    }

    fn __neg__(&self) -> Self {
        (-self.0).into()
    }

    fn __add__(&self, rhs: &Rect) -> Rect {
        (rhs.0 + self.0).into()
    }

    fn __sub__(&self, rhs: &Rect) -> Rect {
        (rhs.0 - self.0).into()
    }

    fn __repr__(&self) -> String {
        // format!("Insets{{ x0:{}, y0: {}, x1: {}, y1: {} }}", self.0.x0, self.0.y0, self.0.x1, self.0.y1)
        format!("{:?}", self.0)

    }

}

impl_isfinitenan!(Insets);