macro_rules! polymorphic {
    (add ($method:expr, $rhs:ty, $returns:ty)+) => {
        
        fn __add__(slf: PyRef<'_, Self>, rhs: &Bound<PyAny>) -> PyResult<PyObject> {
            Python::with_gil(|py| {
                let magic = PyModule::import_bound(py, "kurbopy.magic")?;
                magic.getattr("magic_add")?.call1((slf, rhs))?.extract()
            })
        }

        $( fn $method(&self, rhs: $rhs) -> $returns {
            (self.0 + rhs.0).into()
        }
        )+
    };
}