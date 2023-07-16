plain
    Checking alloc v0.0.0 (/checkout/library/alloc)
    Checking cfg-if v0.1.10
    Checking adler v0.2.3
    Checking rustc-demangle v0.1.21
error[E0658]: use of unstable library feature 'unchecked_math': niche optimization path
     |
     |
2290 |             unsafe { (len.unchecked_mul(N), cap.unchecked_mul(N)) }
     |
     = note: see issue #85122 <https://github.com/rust-lang/rust/issues/85122> for more information
     = note: see issue #85122 <https://github.com/rust-lang/rust/issues/85122> for more information
     = help: add `#![feature(unchecked_math)]` to the crate attributes to enable

error[E0658]: use of unstable library feature 'unchecked_math': niche optimization path
     |
     |
2290 |             unsafe { (len.unchecked_mul(N), cap.unchecked_mul(N)) }
     |
     = note: see issue #85122 <https://github.com/rust-lang/rust/issues/85122> for more information
     = note: see issue #85122 <https://github.com/rust-lang/rust/issues/85122> for more information
     = help: add `#![feature(unchecked_math)]` to the crate attributes to enable
For more information about this error, try `rustc --explain E0658`.
error: could not compile `alloc` due to 2 previous errors
Build completed unsuccessfully in 0:01:18
