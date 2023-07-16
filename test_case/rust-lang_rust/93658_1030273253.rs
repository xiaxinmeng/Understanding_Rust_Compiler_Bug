plain
    Checking rand v0.7.3
    Checking core v0.0.0 (/checkout/library/core)
    Checking alloc v0.0.0 (/checkout/library/alloc)
    Checking std v0.0.0 (/checkout/library/std)
error[E0658]: `cfg(panic)` is experimental and subject to change
    |
    |
395 | #[cfg(not(panic = "abort"))]
    |
    = note: see issue #77443 <https://github.com/rust-lang/rust/issues/77443> for more information
    = help: add `#![feature(cfg_panic)]` to the crate attributes to enable


error[E0658]: `cfg(panic)` is experimental and subject to change
    |
    |
419 | #[cfg(not(panic = "abort"))]
    |
    = note: see issue #77443 <https://github.com/rust-lang/rust/issues/77443> for more information
    = help: add `#![feature(cfg_panic)]` to the crate attributes to enable


error[E0658]: `cfg(panic)` is experimental and subject to change
    |
    |
443 | #[cfg(not(panic = "abort"))]
    |
    = note: see issue #77443 <https://github.com/rust-lang/rust/issues/77443> for more information
    = help: add `#![feature(cfg_panic)]` to the crate attributes to enable


error[E0658]: `cfg(panic)` is experimental and subject to change
    |
    |
236 | #[cfg(panic = "unwind")]
    |
    = note: see issue #77443 <https://github.com/rust-lang/rust/issues/77443> for more information
    = help: add `#![feature(cfg_panic)]` to the crate attributes to enable


error[E0658]: `cfg(panic)` is experimental and subject to change
  |
  |
3 | #[cfg(panic = "unwind")]
  |
  = note: see issue #77443 <https://github.com/rust-lang/rust/issues/77443> for more information
  = help: add `#![feature(cfg_panic)]` to the crate attributes to enable


error[E0658]: `cfg(panic)` is experimental and subject to change
    |
    |
193 | #[cfg(panic = "unwind")]
    |
    = note: see issue #77443 <https://github.com/rust-lang/rust/issues/77443> for more information
    = help: add `#![feature(cfg_panic)]` to the crate attributes to enable

