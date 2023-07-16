
error[E0275]: overflow evaluating the requirement `&_: IntoIterator`
   --> library/test/src/lib.rs:266:21
    |
266 |         for test in &timed_out {
    |                     ^^^^^^^^^^
    |
    = help: consider adding a `#![recursion_limit="256"]` attribute to your crate (`test`)
    = note: required because of the requirements on the impl of `IntoIterator` for `&(_, _)`
    = note: required because of the requirements on the impl of `IntoIterator` for `&((_, _), _)`
    = note: required because of the requirements on the impl of `IntoIterator` for `&(((_, _), _), _)`
[...many more...]
