#[macro_export]
macro_rules! polymorphic {
    (add $target:ty => $(($method:ident, $rhs:ty, $returns:ty)),+) => {
        #[pymethods]
        impl $target {
        
        fn __add__(slf: PyRef<'_, Self>, rhs: &Bound<PyAny>) -> PyResult<PyObject> {
            Python::with_gil(|py| {
                let magic = PyModule::import_bound(py, "kurbopy.magic")?;
                magic.getattr("magic_add")?.call1((slf, rhs))?.extract()
            })
        }

        $( 
            #[allow(non_snake_case)]
            fn $method(&self, rhs: $rhs) -> $returns {
            (self.0 + rhs.0).into()
        }
        )+

        }
    };
    (sub $target:ty => $(($method:ident, $rhs:ty, $returns:ty)),+) => {
        #[pymethods]
        impl $target {
        
        fn __sub__(slf: PyRef<'_, Self>, rhs: &Bound<PyAny>) -> PyResult<PyObject> {
            Python::with_gil(|py| {
                let magic = PyModule::import_bound(py, "kurbopy.magic")?;
                magic.getattr("magic_sub")?.call1((slf, rhs))?.extract()
            })
        }

        $( 
            #[allow(non_snake_case)]
            fn $method(&self, rhs: $rhs) -> $returns {
            (self.0 - rhs.0).into()
        }
        )+

        }
    };
    (mul $target:ty => $(($method:ident, $rhs:ty, $returns:ty)),+) => {
        #[pymethods]
        impl $target {
        fn __mul__(slf: PyRef<'_, Self>, rhs: &Bound<PyAny>) -> PyResult<PyObject> {
            Python::with_gil(|py| {
                let magic = PyModule::import_bound(py, "kurbopy.magic")?;
                magic.getattr("magic_mul")?.call1((slf, rhs))?.extract()
            })
        }

        $( 
            #[allow(non_snake_case)]
            fn $method(&self, rhs: $rhs) -> $returns {
            (self.0 * rhs.0).into()
        }
        )+
        }
    };
}