#[macro_export]
macro_rules! impl_paramcurve {
    ($name:ident) => {
        #[pyo3::prelude::pymethods]
        impl $name {

            /// Evaluate the curve at parameter `t`.
            ///
            /// Generally `t` is in the range [0..1].
            #[pyo3(text_signature = "($self, t)")]
            fn eval(&self, t: f64) -> $crate::point::Point {
                self.0.eval(t).into()
            }

            /// Get a subsegment of the curve for the given parameter range.
            #[pyo3(text_signature = "($self, (t0,t1))")]
            fn subsegment(&self, range: (f64, f64)) -> Self {
                self.0.subsegment(range.0..range.1).into()
            }

            /// The start point.
            fn start(&self) -> $crate::point::Point {
                self.0.start().into()
            }
            /// The end point.
            fn end(&self) -> $crate::point::Point {
                self.0.end().into()
            }

            /// Subdivide into (roughly) halves.
            fn subdivide(&self) -> (Self, Self) {
                let (a, b) = self.0.subdivide();
                (a.into(), b.into())
            }
        }
    }
}

#[macro_export]
macro_rules! impl_paramcurvearclen {
    ($name:ident) => {
        #[pyo3::prelude::pymethods]
        impl $name {

            /// The arc length of the curve.
            ///
            /// The result is accurate to the given accuracy (subject to
            /// roundoff errors for ridiculously low values). Compute time
            /// may vary with accuracy, if the curve needs to be subdivided.
            #[pyo3(text_signature = "($self, accuracy)")]
            fn arclen(&self, accuracy: f64) -> f64 {
                self.0.arclen(accuracy)
            }

            /// Solve for the parameter that has the given arc length from the start.
            ///
            /// This implementation uses the IPT method, as provided by
            /// [`common::solve_itp`]. This is as robust as bisection but
            /// typically converges faster. In addition, the method takes
            /// care to compute arc lengths of increasingly smaller segments
            /// of the curve, as that is likely faster than repeatedly
            /// computing the arc length of the segment starting at t=0.
            #[pyo3(text_signature = "($self, arclen, accuracy)")]
            fn inv_arclen(&self, arclen: f64, accuracy: f64) -> f64 {
                self.0.inv_arclen(arclen, accuracy)
            }
        }
    }

}

#[macro_export]
macro_rules! impl_paramcurvearea {
    ($name:ident) => {
        #[pyo3::prelude::pymethods]
        impl $name {
            /// Compute the signed area under the curve.
            ///
            /// For a closed path, the signed area of the path is the sum of signed
            /// areas of the segments. This is a variant of the "shoelace formula."
            /// See:
            /// <https://github.com/Pomax/bezierinfo/issues/44> and
            /// <http://ich.deanmcnamee.com/graphics/2016/03/30/CurveArea.html>
            ///
            /// This can be computed exactly for Béziers thanks to Green's theorem,
            /// and also for simple curves such as circular arcs. For more exotic
            /// curves, it's probably best to subdivide to cubics. We leave that
            /// to the caller, which is why we don't give an accuracy param here.
            fn signed_area(&self) -> f64 {
                self.0.signed_area()

            }
        }
    }
}


#[macro_export]
macro_rules! impl_paramcurvecurvature {
    ($name:ident) => {
        #[pyo3::prelude::pymethods]
        impl $name {
            // Compute the signed curvature at parameter `t`.
            fn curvature(&self, t: f64) -> f64 {
                self.0.curvature(t)
            }
        }
    }
}

#[macro_export]
macro_rules! impl_paramcurveextrema {
    ($name:ident) => {
        #[pyo3::prelude::pymethods]
        impl $name {
            /// Compute the extrema of the curve.
            ///
            /// Only extrema within the interior of the curve count.
            ///
            /// The extrema should be reported in increasing parameter order.
            fn extrema(&self) -> Vec<f64> {
                self.0.extrema().to_vec()
            }
            /// Return parameter ranges, each of which is monotonic within the range.
            fn extrema_ranges(&self) -> Vec<(f64, f64)> {
                self.0.extrema_ranges().iter().map(|r| (r.start, r.end)).collect()
            }
            /// The smallest rectangle that encloses the curve in the range (0..1).
            fn bounding_box(&self) -> $crate::rect::Rect {
                kurbo::ParamCurveExtrema::bounding_box(&self.0).into()
            }
        }
    }
}

#[macro_export]
macro_rules! impl_paramcurvenearest {
    ($name:ident) => {
        #[pyo3::prelude::pymethods]
        impl $name {

        /// Find the position on the curve that is nearest to the given point.
        ///
        /// This returns a [`Nearest`] struct that contains information about the position.
        #[pyo3(text_signature = "($self, point, accuracy)")]
        fn nearest(&self, p: Point, accuracy: f64) -> Nearest {
            let n = self.0.nearest(p.0, accuracy);
            n.into()
        }
    }
}
}

#[macro_export]
macro_rules! impl_paramcurvederiv {
    ($name:ident, $target:ident) => {
        #[pyo3::prelude::pymethods]
        impl $name {
            /// The derivative of the curve.
            ///
            /// Note that the type of the return value is somewhat inaccurate, as
            /// the derivative of a curve (mapping of param to point) is a mapping
            /// of param to vector. We choose to accept this rather than have a
            /// more complex type scheme.
            pub fn deriv(&self) -> $target {
                self.0.deriv().into()
            }

        }
    }
}

