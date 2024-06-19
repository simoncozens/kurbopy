use kurbo::common;

use pyo3::prelude::*;

#[pyfunction]
/// Factor a quartic into two quadratics.
///
/// Attempt to factor a quartic equation into two quadratic equations. Returns `None` either if there
/// is overflow (in which case rescaling might succeed) or the factorization would result in
/// complex coefficients.
///
/// Discussion question: distinguish the two cases in return value?
pub fn factor_quartic_inner(
    _py: Python,
    a: f64,
    b: f64,
    c: f64,
    d: f64,
    rescale: bool,
) -> Option<Vec<(f64, f64)>> {
    let roots = common::factor_quartic_inner(a, b, c, d, rescale);
    roots.map(|g| g.to_vec())
}

#[pyfunction]
/// Find real roots of cubic equation.
///
/// The implementation is not (yet) fully robust, but it does handle the case
/// where `c3` is zero (in that case, solving the quadratic equation).
///
/// See: <https://momentsingraphics.de/CubicRoots.html>
///
/// That implementation is in turn based on Jim Blinn's "How to Solve a Cubic
/// Equation", which is masterful.
///
/// Return values of x for which c0 + c1 x + c2 x² + c3 x³ = 0.
pub fn solve_cubic(c0: f64, c1: f64, c2: f64, c3: f64) -> Vec<f64> {
    common::solve_cubic(c0, c1, c2, c3).to_vec()
}

/// Solve an arbitrary function for a zero-crossing.
///
/// This uses the [ITP method], as described in the paper
/// [An Enhancement of the Bisection Method Average Performance Preserving Minmax Optimality].
///
/// The values of `ya` and `yb` are given as arguments rather than
/// computed from `f`, as the values may already be known, or they may
/// be less expensive to compute as special cases.
///
/// It is assumed that `ya < 0.0` and `yb > 0.0`, otherwise unexpected
/// results may occur.
///
/// The value of `epsilon` must be larger than 2^-63 times `b - a`,
/// otherwise integer overflow may occur. The `a` and `b` parameters
/// represent the lower and upper bounds of the bracket searched for a
/// solution.
///
/// The ITP method has tuning parameters. This implementation hardwires
/// k2 to 2, both because it avoids an expensive floating point
/// exponentiation, and because this value has been tested to work well
/// with curve fitting problems.
///
/// The `n0` parameter controls the relative impact of the bisection and
/// secant components. When it is 0, the number of iterations is
/// guaranteed to be no more than the number required by bisection (thus,
/// this method is strictly superior to bisection). However, when the
/// function is smooth, a value of 1 gives the secant method more of a
/// chance to engage, so the average number of iterations is likely
/// lower, though there can be one more iteration than bisection in the
/// worst case.
///
/// The `k1` parameter is harder to characterize, and interested users
/// are referred to the paper, as well as encouraged to do empirical
/// testing. To match the paper, a value of `0.2 / (b - a)` is
/// suggested, and this is confirmed to give good results.
///
/// When the function is monotonic, the returned result is guaranteed to
/// be within `epsilon` of the zero crossing. For more detailed analysis,
/// again see the paper.
///
/// [ITP method]: https://en.wikipedia.org/wiki/ITP_Method
/// [An Enhancement of the Bisection Method Average Performance Preserving Minmax Optimality]: https://dl.acm.org/doi/10.1145/3423597
#[allow(clippy::too_many_arguments)]
#[pyfunction]
pub fn solve_itp(
    py: Python,
    fun: Py<PyAny>,
    a: f64,
    b: f64,
    epsilon: f64,
    n0: usize,
    k1: f64,
    ya: f64,
    yb: f64,
) -> PyResult<f64> {
    Ok(common::solve_itp(
        |x: f64| {
            fun.call1(py, (x,))
                .and_then(|pyr| pyr.extract::<f64>(py))
                .unwrap_or(f64::NAN)
        },
        a,
        b,
        epsilon,
        n0,
        k1,
        ya,
        yb,
    ))
}

/// Find real roots of quadratic equation.
///
/// Return values of x for which c0 + c1 x + c2 x² = 0.
///
/// This function tries to be quite numerically robust. If the equation
/// is nearly linear, it will return the root ignoring the quadratic term;
/// the other root might be out of representable range. In the degenerate
/// case where all coefficients are zero, so that all values of x satisfy
/// the equation, a single `0.0` is returned.
#[pyfunction]
pub fn solve_quadratic(c0: f64, c1: f64, c2: f64) -> Vec<f64> {
    common::solve_quadratic(c0, c1, c2).to_vec()
}

/// Find real roots of a quartic equation.
///
/// This is a fairly literal implementation of the method described in:
/// Algorithm 1010: Boosting Efficiency in Solving Quartic Equations with
/// No Compromise in Accuracy, Orellana and De Michele, ACM
/// Transactions on Mathematical Software, Vol. 46, No. 2, May 2020.
#[pyfunction]
pub fn solve_quartic(c0: f64, c1: f64, c2: f64, c3: f64, c4: f64) -> Vec<f64> {
    common::solve_quartic(c0, c1, c2, c3, c4).to_vec()
}

#[macro_export]
macro_rules! impl_isfinitenan {
    ($name:ident) => {
        #[pymethods]
        impl $name {
            /// Is this value finite?
            fn is_finite(&self) -> bool {
                self.0.is_finite()
            }
            /// Is this value NaN?
            fn is_nan(&self) -> bool {
                self.0.is_nan()
            }
        }
    };
}