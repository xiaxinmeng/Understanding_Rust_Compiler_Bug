plain
    Checking rustc-demangle v0.1.21
error[E0433]: failed to resolve: unresolved import
    --> library/alloc/src/vec/mod.rs:2281:33
     |
2281 |         let new_len = if crate::mem::size_of::<T>() == 0 {
     |                                 |
     |                                 unresolved import
     |                                 help: a similar path exists: `compiler_builtins::mem`


error[E0658]: use of unstable library feature 'unchecked_math': niche optimization path
     |
     |
2286 |             unsafe { self.len().unchecked_mul(N) }
     |
     = note: see issue #85122 <https://github.com/rust-lang/rust/issues/85122> for more information
     = note: see issue #85122 <https://github.com/rust-lang/rust/issues/85122> for more information
     = help: add `#![feature(unchecked_math)]` to the crate attributes to enable
Some errors have detailed explanations: E0433, E0658.
For more information about an error, try `rustc --explain E0433`.
error: could not compile `alloc` due to 2 previous errors
Build completed unsuccessfully in 0:01:07
