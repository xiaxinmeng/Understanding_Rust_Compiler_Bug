console
$ cargo +nightly check -Z unstable-options -Z check-cfg=features
warning: unexpected `cfg` condition value
 --> src/main.rs:6:11
  |
6 | #[cfg(any(feature = "feature_a", feature = "feature-b"))]
  |           ^^^^^^^^^^^^^^^^^^^^^
  |
  = note: no expected value for `feature`
  = note: `#[warn(unexpected_cfgs)]` on by default

warning: unexpected `cfg` condition value
 --> src/main.rs:6:34
  |
6 | #[cfg(any(feature = "feature_a", feature = "feature-b"))]
  |                                  ^^^^^^^^^^^^^^^^^^^^^
  |
  = note: no expected value for `feature`

warning: unexpected `cfg` condition value
 --> src/main.rs:2:15
  |
2 |     #[cfg(any(feature = "feature_a", feature = "feature-b"))]
  |               ^^^^^^^^^^^^^^^^^^^^^
  |
  = note: no expected value for `feature`

warning: unexpected `cfg` condition value
 --> src/main.rs:2:38
  |
2 |     #[cfg(any(feature = "feature_a", feature = "feature-b"))]
  |                                      ^^^^^^^^^^^^^^^^^^^^^
  |
  = note: no expected value for `feature`

warning: `f` (bin "f") generated 4 warnings
    Finished dev [unoptimized + debuginfo] target(s) in 0.19s
