#[macro_export]
macro_rules! polymorphic {
    (add $target:ty => $(($method:ident, $rhs:ty, $returns:ty)),+) => {
        #[pymethods]
        impl $target {

        fn __add__(slf: PyRef<'_, Self>, rhs: &Bound<PyAny>) -> PyResult<Py<PyAny>> {
            Python::attach(|py| {
                let magic = PyModule::import(py, "kurbopy.magic")?;
                Ok(magic.getattr("magic_add")?.call1((slf, rhs))?
                .into_pyobject(py)?.unbind())
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

        fn __sub__(slf: PyRef<'_, Self>, rhs: &Bound<PyAny>) -> PyResult<Py<PyAny>> {
            Python::attach(|py| {
                let magic = PyModule::import(py, "kurbopy.magic")?;
                Ok(magic.getattr("magic_sub")?.call1((slf, rhs))?.into_pyobject(py)?.unbind())
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
        fn __mul__(slf: PyRef<'_, Self>, rhs: &Bound<PyAny>) -> PyResult<Py<PyAny>> {
            Python::attach(|py| {
                let magic = PyModule::import(py, "kurbopy.magic")?;
                let foo = magic.getattr("magic_mul")?.call1((slf, rhs))?;
                Ok(foo.into_pyobject(py)?.unbind())
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
