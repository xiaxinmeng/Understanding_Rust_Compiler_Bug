rust
[cfg(not(feature = "nightly"))]
unsafe trait ZerosValid {}

[cfg(feature = "nightly")]
unsafe auto trait ZerosValid {}
