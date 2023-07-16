rust
error[E0275]: overflow evaluating the requirement `ops::range::RangeFull: ops::range::RangeBounds<&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&_>`
  |
  = help: consider adding a `#![recursion_limit="128"]` attribute to your crate
  = note: required because of the requirements on the impl of `ops::range::RangeBounds<&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&_>` for `ops::range::RangeFull`
  = note: required because of the requirements on the impl of `ops::range::RangeBounds<&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&_>` for `ops::range::RangeFull`
[…]
  = note: required because of the requirements on the impl of `ops::range::RangeBounds<&&_>` for `ops::range::RangeFull`
  = note: required because of the requirements on the impl of `ops::range::RangeBounds<&_>` for `ops::range::RangeFull`

error: aborting due to previous error
