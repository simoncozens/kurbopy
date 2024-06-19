#[macro_export]
macro_rules! impl_shape {
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
        fn area(&self) -> f64 {
            self.0.area()
        }

        /// Total length of perimeter.
        #[pyo3(text_signature = "($self, accuracy)")]
        fn perimeter(&self, accuracy: f64) -> f64 {
            self.0.perimeter(accuracy)
        }

        /// The winding number of a point.
        ///
        /// This method only produces meaningful results with closed shapes.
        ///
        /// The sign of the winding number is consistent with that of ``area``,
        /// meaning it is +1 when the point is inside a positive area shape
        /// and -1 when it is inside a negative area shape. Of course, greater
        /// magnitude values are also possible when the shape is more complex.
        #[pyo3(text_signature = "($self, pt)")]
        fn winding(&self, pt: Point) -> i32 {
            self.0.winding(pt.0)
        }

        /// The smallest rectangle that encloses the shape.
        fn bounding_box(&self) -> Rect {
            Shape::bounding_box(&self.0).into()
        }

        /// Returns `true` if the [`Point`] is inside this shape.
        ///
        /// This is only meaningful for closed shapes.
        fn contains(&self, pt: Point) -> bool {
            self.winding(pt) != 0
        }

        /// Convert to a Bézier path.
        fn to_path(&self, tolerance: f64) -> $crate::bezpath::BezPath {
            self.0.to_path(tolerance).into()
        }
    }
}
}

#[macro_export]
macro_rules! impl_shape_no_bounding_box {
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
        fn area(&self) -> f64 {
            kurbo::Shape::area(&self.0)
        }

        /// Total length of perimeter.
        #[pyo3(text_signature = "($self, accuracy)")]
        fn perimeter(&self, accuracy: f64) -> f64 {
            kurbo::Shape::perimeter(&self.0, accuracy)
        }

        /// The winding number of a point.
        ///
        /// This method only produces meaningful results with closed shapes.
        ///
        /// The sign of the winding number is consistent with that of ``area``,
        /// meaning it is +1 when the point is inside a positive area shape
        /// and -1 when it is inside a negative area shape. Of course, greater
        /// magnitude values are also possible when the shape is more complex.
        #[pyo3(text_signature = "($self, pt)")]
        fn winding(&self, pt: Point) -> i32 {
            kurbo::Shape::winding(&self.0, pt.0)
        }

        // /// The smallest rectangle that encloses the shape.
        // fn bounding_box(&self) -> Rect {
        //     Shape::bounding_box(&self.0).into()
        // }

        /// Returns `true` if the [`Point`] is inside this shape.
        ///
        /// This is only meaningful for closed shapes.
        fn contains(&self, pt: Point) -> bool {
            kurbo::Shape::contains(&self.0, pt.0)
        }

        /// Convert to a Bézier path.
        fn to_path(&self, tolerance: f64) -> $crate::bezpath::BezPath {
            kurbo::Shape::to_path(&self.0, tolerance).into()
        }
    }
}
}