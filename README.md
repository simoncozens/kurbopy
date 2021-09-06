# kurbopy

Kurbopy is a Python wrapper around the Rust [kurbo](https://github.com/linebender/kurbo)
library, a 2D curve manipulation library with "a focus on accuracy and good
performance in high-accuracy conditions".

Kurbopy is currently an incomplete wrapper, with additional
methods and classes being added on as an-needed basis. If you need a method
or class from kurbo added to kurbopy, please ask for it in the issue tracker.

## Building

Use `maturin` to build `kurbopy`.

```
pip3 install maturin
maturin develop # In a virtualenv
maturin build # Build wheel
```
