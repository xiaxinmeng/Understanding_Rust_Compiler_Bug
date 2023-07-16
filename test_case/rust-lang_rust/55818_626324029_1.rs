
warning: unused variable: `py`
  --> src/lib.rs:16:12
   |
16 | fn rust_py(py: Python, m: &PyModule) -> PyResult<()> {
   |            ^^ help: if this is intentional, prefix it with an underscore: `_py`
   |
   = note: `#[warn(unused_variables)]` on by default

warning: 1 warning emitted
