
error[E0275]: overflow evaluating the requirement `[_; 0]: Default`
  |
  = help: consider adding a `#![recursion_limit="256"]` attribute to your crate (`default-demo`)
  = note: required because of the requirements on the impl of `Default` for `[_; 0]`
  = note: 127 redundant requirements hidden
  = note: required because of the requirements on the impl of `Default` for `[_; 0]`
