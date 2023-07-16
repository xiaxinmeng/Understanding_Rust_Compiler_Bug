plain
[00:04:34]    Compiling rustc_asan v0.0.0 (file:///checkout/src/librustc_asan)
[00:04:35]    Compiling rustc_msan v0.0.0 (file:///checkout/src/librustc_msan)
[00:04:35]    Compiling rustc_lsan v0.0.0 (file:///checkout/src/librustc_lsan)
[00:04:36]    Compiling rustc_tsan v0.0.0 (file:///checkout/src/librustc_tsan)
[00:04:37] error[E0615]: attempted to take value of method `align` on type `&alloc::Layout`
[00:04:37]    --> libcore/alloc.rs:251:68
[00:04:37]     |
[00:04:37] 251 |             Ok((Layout::from_size_align_unchecked(alloc_size, self.align)?, padded_size))
[00:04:37]     |
[00:04:37]     |
[00:04:37]     = help: maybe a `()` to call it is missing?
[00:04:37] 
[00:04:37] error[E0277]: the `?` operator can only be applied to values that implement `ops::try::Try`
[00:04:37]    --> libcore/alloc.rs:251:17
[00:04:37]     |
[00:04:37] 251 |             Ok((Layout::from_size_align_unchecked(alloc_size, self.align)?, padded_size))
[00:04:37]     |                 ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the `?` operator cannot be applied to type `alloc::Layout`
[00:04:37]     |
[00:04:37]     = help: the trait `ops::try::Try` is not implemented for `alloc::Layout`
[00:04:37] note: required by `ops::try::Try::into_result`
[00:04:37]    --> libcore/ops/try.rs:50:5
[00:04:37]     |
[00:04:37] 50  |     fn into_result(self) -> Result<Self::Ok, Self::Error>;
[00:04:37] 
[00:04:37] error: aborting due to 2 previous errors
[00:04:37] 
[00:04:37] Some errors occurred: E0277, E0615.
