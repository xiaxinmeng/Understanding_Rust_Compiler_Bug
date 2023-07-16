rust
[cfg(not(feature = "nightly"))]
macro_rules! declare_zeroes_valid {
    unsafe trait ZerosValid {}
}


[cfg(feature = "nightly")]
macro_rules! declare_zeroes_valid {
    unsafe auto trait ZerosValid {}
}

declare_zeroes_valid!()
