# kurbopy

[![documentation](https://readthedocs.org/projects/kurbopy/badge/)](https://kurbopy.readthedocs.io)
[![pypi](https://img.shields.io/pypi/v/kurbopy)](https://pypi.org/project/kurbopy/)
![github actions](https://img.shields.io/github/workflow/status/simoncozens/kurbopy/CI)

Kurbopy is a Python wrapper around the Rust [kurbo](https://github.com/linebender/kurbo)
library, a 2D curve manipulation library with "a focus on accuracy and good
performance in high-accuracy conditions".

Kurbopy is currently an incomplete wrapper, with additional
methods and classes being added on as an-needed basis. If you need a method
or class from kurbo added to kurbopy, please ask for it in the issue tracker.

The version number of kurbopy releases is formed of the kurbo version number
with the patch number multiplied by ten plus any kurbopy patch releases.
In other words, kurbopy version 0.8.15 is the fifth patch release tracking
kurbo 0.8.1.

## Building

Use `maturin` to build `kurbopy`.

```
pip3 install maturin
maturin develop # In a virtualenv
maturin build # Build wheel
```
