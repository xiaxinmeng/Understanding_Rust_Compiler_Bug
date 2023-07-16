plain
  0     0    0     0    0     0      0      0 --:--:-- --:--:-- --:--:--     0
100  4234  100  4234    0     0  25506      0 --:--:-- --:--:-- --:--:-- 25506
+ RUSTC_BOOTSTRAP=1
+ ./build/x86_64-unknown-linux-gnu/stage2/bin/rustc --edition=2018 --crate-type=lib /tmp/ctfe-stress-4.rs
error[E0658]: trait bounds other than `Sized` on const fn parameters are unstable
   |
   |
67 | expensive_static!(UNSIZE_TRAIT: &'static dyn Trait = &42u32; [4 16 16 16 16 16]);
   |
   = note: see issue #57563 <https://github.com/rust-lang/rust/issues/57563> for more information
   = help: add `#![feature(const_fn_trait_bound)]` to the crate attributes to enable


error[E0658]: unsizing casts to types besides slices are not allowed in const fn
   |
   |
28 |         e(0); e(0); e(0); e(0)
...
...
67 | expensive_static!(UNSIZE_TRAIT: &'static dyn Trait = &42u32; [4 16 16 16 16 16]);
   |
   = note: see issue #64992 <https://github.com/rust-lang/rust/issues/64992> for more information
   = help: add `#![feature(const_fn_unsize)]` to the crate attributes to enable
   = note: this error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)
   = note: this error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)

error[E0658]: unsizing casts to types besides slices are not allowed in const fn
   |
   |
23 |       ([16 $($n: tt)*] $e: expr, $T: ty) => {{
   |  ____________________________________________^
24 | |         const fn e(_: u32) -> $T { const_repeat!([$($n)*] $e, $T) }
25 | |         e(0); e(0); e(0); e(0);
26 | |         e(0); e(0); e(0); e(0);
27 | |         e(0); e(0); e(0); e(0);
28 | |         e(0); e(0); e(0); e(0)
29 | |     }};
...
...
67 |   expensive_static!(UNSIZE_TRAIT: &'static dyn Trait = &42u32; [4 16 16 16 16 16]);
   |
   = note: see issue #64992 <https://github.com/rust-lang/rust/issues/64992> for more information
   = help: add `#![feature(const_fn_unsize)]` to the crate attributes to enable
   = note: this error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)
   = note: this error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)

error[E0658]: unsizing casts to types besides slices are not allowed in const fn
   |
   |
28 |         e(0); e(0); e(0); e(0)
...
...
67 | expensive_static!(UNSIZE_TRAIT: &'static dyn Trait = &42u32; [4 16 16 16 16 16]);
   |
   = note: see issue #64992 <https://github.com/rust-lang/rust/issues/64992> for more information
   = help: add `#![feature(const_fn_unsize)]` to the crate attributes to enable
   = note: this error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)
   = note: this error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)

error[E0658]: unsizing casts to types besides slices are not allowed in const fn
   |
   |
23 |       ([16 $($n: tt)*] $e: expr, $T: ty) => {{
   |  ____________________________________________^
24 | |         const fn e(_: u32) -> $T { const_repeat!([$($n)*] $e, $T) }
25 | |         e(0); e(0); e(0); e(0);
26 | |         e(0); e(0); e(0); e(0);
27 | |         e(0); e(0); e(0); e(0);
28 | |         e(0); e(0); e(0); e(0)
29 | |     }};
...
...
67 |   expensive_static!(UNSIZE_TRAIT: &'static dyn Trait = &42u32; [4 16 16 16 16 16]);
   |
   = note: see issue #64992 <https://github.com/rust-lang/rust/issues/64992> for more information
   = help: add `#![feature(const_fn_unsize)]` to the crate attributes to enable
   = note: this error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)
   = note: this error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)

error[E0658]: unsizing casts to types besides slices are not allowed in const fn
   |
   |
28 |         e(0); e(0); e(0); e(0)
...
...
67 | expensive_static!(UNSIZE_TRAIT: &'static dyn Trait = &42u32; [4 16 16 16 16 16]);
   |
   = note: see issue #64992 <https://github.com/rust-lang/rust/issues/64992> for more information
   = help: add `#![feature(const_fn_unsize)]` to the crate attributes to enable
   = note: this error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)
   = note: this error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)

error[E0658]: unsizing casts to types besides slices are not allowed in const fn
   |
   |
23 |       ([16 $($n: tt)*] $e: expr, $T: ty) => {{
   |  ____________________________________________^
24 | |         const fn e(_: u32) -> $T { const_repeat!([$($n)*] $e, $T) }
25 | |         e(0); e(0); e(0); e(0);
26 | |         e(0); e(0); e(0); e(0);
27 | |         e(0); e(0); e(0); e(0);
28 | |         e(0); e(0); e(0); e(0)
29 | |     }};
...
...
67 |   expensive_static!(UNSIZE_TRAIT: &'static dyn Trait = &42u32; [4 16 16 16 16 16]);
   |
   = note: see issue #64992 <https://github.com/rust-lang/rust/issues/64992> for more information
   = help: add `#![feature(const_fn_unsize)]` to the crate attributes to enable
   = note: this error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)
   = note: this error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)

error[E0658]: unsizing casts to types besides slices are not allowed in const fn
   |
   |
67 | expensive_static!(UNSIZE_TRAIT: &'static dyn Trait = &42u32; [4 16 16 16 16 16]);
   |
   = note: see issue #64992 <https://github.com/rust-lang/rust/issues/64992> for more information
   = help: add `#![feature(const_fn_unsize)]` to the crate attributes to enable


error[E0658]: unsizing casts to types besides slices are not allowed in const fn
   |
   |
13 |       ([16] $e: expr, $T: ty) => {{
   |  _________________________________^
14 | |         $e; $e; $e; $e;
15 | |         $e; $e; $e; $e;
16 | |         $e; $e; $e; $e;
17 | |         $e; $e; $e; $e
18 | |     }};
...
...
67 |   expensive_static!(UNSIZE_TRAIT: &'static dyn Trait = &42u32; [4 16 16 16 16 16]);
   |
   = note: see issue #64992 <https://github.com/rust-lang/rust/issues/64992> for more information
   = help: add `#![feature(const_fn_unsize)]` to the crate attributes to enable
   = note: this error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)
   = note: this error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)

error[E0658]: unsizing casts to types besides slices are not allowed in const fn
   |
   |
28 |         e(0); e(0); e(0); e(0)
...
...
67 | expensive_static!(UNSIZE_TRAIT: &'static dyn Trait = &42u32; [4 16 16 16 16 16]);
   |
   = note: see issue #64992 <https://github.com/rust-lang/rust/issues/64992> for more information
   = help: add `#![feature(const_fn_unsize)]` to the crate attributes to enable
   = note: this error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)
   = note: this error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)

error[E0658]: unsizing casts to types besides slices are not allowed in const fn
   |
   |
23 |       ([16 $($n: tt)*] $e: expr, $T: ty) => {{
   |  ____________________________________________^
24 | |         const fn e(_: u32) -> $T { const_repeat!([$($n)*] $e, $T) }
25 | |         e(0); e(0); e(0); e(0);
26 | |         e(0); e(0); e(0); e(0);
27 | |         e(0); e(0); e(0); e(0);
28 | |         e(0); e(0); e(0); e(0)
29 | |     }};
...
...
67 |   expensive_static!(UNSIZE_TRAIT: &'static dyn Trait = &42u32; [4 16 16 16 16 16]);
   |
   = note: see issue #64992 <https://github.com/rust-lang/rust/issues/64992> for more information
   = help: add `#![feature(const_fn_unsize)]` to the crate attributes to enable
   = note: this error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)
