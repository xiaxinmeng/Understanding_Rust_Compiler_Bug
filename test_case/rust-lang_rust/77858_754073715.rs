plain
    Checking rustc-rayon v0.3.0
    Checking tempfile v3.1.0
    Checking regex v1.3.9
    Checking rustdoc v0.0.0 (/checkout/src/librustdoc)
error: the feature `split_inclusive` has been stable since 1.49.0 and no longer requires an attribute to enable
   |
18 | #![feature(split_inclusive)]
   |            ^^^^^^^^^^^^^^^
   |
   |
   = note: `-D stable-features` implied by `-D warnings`
error: aborting due to previous error

error: could not compile `rustdoc`

