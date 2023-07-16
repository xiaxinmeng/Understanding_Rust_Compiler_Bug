plain
   Compiling unic-emoji-char v0.9.0
error[E0658]: use of unstable library feature 'stdsimd'
   --> /checkout/library/stdarch/crates/std_detect/src/detect/macros.rs:21:25
    |
17  |   /         macro_rules! $macro_name {
18  |   |             $(
19  |   |                 ($feature_lit) => {
20  |   |                     cfg!(target_feature = $feature_lit) ||
21  |  /|                         $crate::detect::__is_feature_detected::$feature()
22  |  ||                 };
23  |  ||             )*
24  |  ||             $(
47  |  ||             };
48  |  ||         }
48  |  ||         }
    |  ||_________- in this expansion of `is_x86_feature_detected!` (#2)
127 |  |                  }
128 |  |              }
    |  |_____________________________________________________________________^
    |
    |
   ::: /cargo/registry/src/github.com-1ecc6299db9ec823/memchr-2.4.1/src/memchr/x86/mod.rs:35:1
    |
35  | /   macro_rules! unsafe_ifunc {
36  | |       ($fnty:ty, $name:ident, $haystack:ident, $($needle:ident),+) => {{
37  | |           use std::{mem, sync::atomic::{AtomicPtr, Ordering}};
...   |
...   |
45  | |                   if cfg!(memchr_runtime_avx) && is_x86_feature_detected!("avx2") {
...   |
70  | |       }}
71  | |   }
71  | |   }
    | |___- in this expansion of `unsafe_ifunc!` (#1)
...
96  |         unsafe_ifunc!(fn(u8, &[u8]) -> Option<usize>, memchr, haystack, n1)
    |
    = note: see issue #27731 <https://github.com/rust-lang/rust/issues/27731> for more information
    = help: add `#![feature(stdsimd)]` to the crate attributes to enable


error[E0658]: use of unstable library feature 'stdsimd'
   --> /checkout/library/stdarch/crates/std_detect/src/detect/macros.rs:21:25
    |
17  |   /         macro_rules! $macro_name {
18  |   |             $(
19  |   |                 ($feature_lit) => {
20  |   |                     cfg!(target_feature = $feature_lit) ||
21  |  /|                         $crate::detect::__is_feature_detected::$feature()
22  |  ||                 };
23  |  ||             )*
24  |  ||             $(
47  |  ||             };
48  |  ||         }
48  |  ||         }
    |  ||_________- in this expansion of `is_x86_feature_detected!` (#2)
127 |  |                  }
128 |  |              }
    |  |_____________________________________________________________________^
    |
    |
   ::: /cargo/registry/src/github.com-1ecc6299db9ec823/memchr-2.4.1/src/memchr/x86/mod.rs:35:1
    |
35  | /   macro_rules! unsafe_ifunc {
36  | |       ($fnty:ty, $name:ident, $haystack:ident, $($needle:ident),+) => {{
37  | |           use std::{mem, sync::atomic::{AtomicPtr, Ordering}};
...   |
...   |
45  | |                   if cfg!(memchr_runtime_avx) && is_x86_feature_detected!("avx2") {
...   |
70  | |       }}
71  | |   }
71  | |   }
    | |___- in this expansion of `unsafe_ifunc!` (#1)
101 | /       unsafe_ifunc!(
101 | /       unsafe_ifunc!(
102 | |           fn(u8, u8, &[u8]) -> Option<usize>,
103 | |           memchr2,
104 | |           haystack,
106 | |           n2
107 | |       )
    | |_______- in this macro invocation (#1)
    |
    |
    = note: see issue #27731 <https://github.com/rust-lang/rust/issues/27731> for more information
    = help: add `#![feature(stdsimd)]` to the crate attributes to enable

error[E0658]: use of unstable library feature 'stdsimd'
   --> /checkout/library/stdarch/crates/std_detect/src/detect/macros.rs:21:25
    |
17  |   /         macro_rules! $macro_name {
18  |   |             $(
19  |   |                 ($feature_lit) => {
20  |   |                     cfg!(target_feature = $feature_lit) ||
21  |  /|                         $crate::detect::__is_feature_detected::$feature()
22  |  ||                 };
23  |  ||             )*
24  |  ||             $(
47  |  ||             };
48  |  ||         }
48  |  ||         }
    |  ||_________- in this expansion of `is_x86_feature_detected!` (#2)
127 |  |                  }
128 |  |              }
    |  |_____________________________________________________________________^
    |
    |
   ::: /cargo/registry/src/github.com-1ecc6299db9ec823/memchr-2.4.1/src/memchr/x86/mod.rs:35:1
    |
35  | /   macro_rules! unsafe_ifunc {
36  | |       ($fnty:ty, $name:ident, $haystack:ident, $($needle:ident),+) => {{
37  | |           use std::{mem, sync::atomic::{AtomicPtr, Ordering}};
...   |
...   |
45  | |                   if cfg!(memchr_runtime_avx) && is_x86_feature_detected!("avx2") {
...   |
70  | |       }}
71  | |   }
71  | |   }
    | |___- in this expansion of `unsafe_ifunc!` (#1)
112 | /       unsafe_ifunc!(
112 | /       unsafe_ifunc!(
113 | |           fn(u8, u8, u8, &[u8]) -> Option<usize>,
114 | |           memchr3,
115 | |           haystack,
118 | |           n3
119 | |       )
    | |_______- in this macro invocation (#1)
    |
    |
    = note: see issue #27731 <https://github.com/rust-lang/rust/issues/27731> for more information
    = help: add `#![feature(stdsimd)]` to the crate attributes to enable

error[E0658]: use of unstable library feature 'stdsimd'
   --> /checkout/library/stdarch/crates/std_detect/src/detect/macros.rs:21:25
    |
17  |   /         macro_rules! $macro_name {
18  |   |             $(
19  |   |                 ($feature_lit) => {
20  |   |                     cfg!(target_feature = $feature_lit) ||
21  |  /|                         $crate::detect::__is_feature_detected::$feature()
22  |  ||                 };
23  |  ||             )*
24  |  ||             $(
47  |  ||             };
48  |  ||         }
48  |  ||         }
    |  ||_________- in this expansion of `is_x86_feature_detected!` (#2)
127 |  |                  }
128 |  |              }
    |  |_____________________________________________________________________^
    |
    |
   ::: /cargo/registry/src/github.com-1ecc6299db9ec823/memchr-2.4.1/src/memchr/x86/mod.rs:35:1
    |
35  | /   macro_rules! unsafe_ifunc {
36  | |       ($fnty:ty, $name:ident, $haystack:ident, $($needle:ident),+) => {{
37  | |           use std::{mem, sync::atomic::{AtomicPtr, Ordering}};
...   |
...   |
45  | |                   if cfg!(memchr_runtime_avx) && is_x86_feature_detected!("avx2") {
...   |
70  | |       }}
71  | |   }
71  | |   }
    | |___- in this expansion of `unsafe_ifunc!` (#1)
...
124 |         unsafe_ifunc!(fn(u8, &[u8]) -> Option<usize>, memrchr, haystack, n1)
    |
    = note: see issue #27731 <https://github.com/rust-lang/rust/issues/27731> for more information
    = help: add `#![feature(stdsimd)]` to the crate attributes to enable


error[E0658]: use of unstable library feature 'stdsimd'
   --> /checkout/library/stdarch/crates/std_detect/src/detect/macros.rs:21:25
    |
17  |   /         macro_rules! $macro_name {
18  |   |             $(
19  |   |                 ($feature_lit) => {
20  |   |                     cfg!(target_feature = $feature_lit) ||
21  |  /|                         $crate::detect::__is_feature_detected::$feature()
22  |  ||                 };
23  |  ||             )*
24  |  ||             $(
47  |  ||             };
48  |  ||         }
48  |  ||         }
    |  ||_________- in this expansion of `is_x86_feature_detected!` (#2)
127 |  |                  }
128 |  |              }
    |  |_____________________________________________________________________^
    |
    |
   ::: /cargo/registry/src/github.com-1ecc6299db9ec823/memchr-2.4.1/src/memchr/x86/mod.rs:35:1
    |
35  | /   macro_rules! unsafe_ifunc {
36  | |       ($fnty:ty, $name:ident, $haystack:ident, $($needle:ident),+) => {{
37  | |           use std::{mem, sync::atomic::{AtomicPtr, Ordering}};
...   |
...   |
45  | |                   if cfg!(memchr_runtime_avx) && is_x86_feature_detected!("avx2") {
...   |
70  | |       }}
71  | |   }
71  | |   }
    | |___- in this expansion of `unsafe_ifunc!` (#1)
129 | /       unsafe_ifunc!(
129 | /       unsafe_ifunc!(
130 | |           fn(u8, u8, &[u8]) -> Option<usize>,
131 | |           memrchr2,
132 | |           haystack,
134 | |           n2
135 | |       )
    | |_______- in this macro invocation (#1)
    |
    |
    = note: see issue #27731 <https://github.com/rust-lang/rust/issues/27731> for more information
    = help: add `#![feature(stdsimd)]` to the crate attributes to enable

error[E0658]: use of unstable library feature 'stdsimd'
   --> /checkout/library/stdarch/crates/std_detect/src/detect/macros.rs:21:25
    |
17  |   /         macro_rules! $macro_name {
18  |   |             $(
19  |   |                 ($feature_lit) => {
20  |   |                     cfg!(target_feature = $feature_lit) ||
21  |  /|                         $crate::detect::__is_feature_detected::$feature()
22  |  ||                 };
23  |  ||             )*
24  |  ||             $(
47  |  ||             };
48  |  ||         }
48  |  ||         }
    |  ||_________- in this expansion of `is_x86_feature_detected!` (#2)
127 |  |                  }
128 |  |              }
    |  |_____________________________________________________________________^
    |
    |
   ::: /cargo/registry/src/github.com-1ecc6299db9ec823/memchr-2.4.1/src/memchr/x86/mod.rs:35:1
    |
35  | /   macro_rules! unsafe_ifunc {
36  | |       ($fnty:ty, $name:ident, $haystack:ident, $($needle:ident),+) => {{
37  | |           use std::{mem, sync::atomic::{AtomicPtr, Ordering}};
...   |
...   |
45  | |                   if cfg!(memchr_runtime_avx) && is_x86_feature_detected!("avx2") {
...   |
70  | |       }}
71  | |   }
71  | |   }
    | |___- in this expansion of `unsafe_ifunc!` (#1)
140 | /       unsafe_ifunc!(
140 | /       unsafe_ifunc!(
141 | |           fn(u8, u8, u8, &[u8]) -> Option<usize>,
142 | |           memrchr3,
143 | |           haystack,
146 | |           n3
147 | |       )
    | |_______- in this macro invocation (#1)
    |
    |
    = note: see issue #27731 <https://github.com/rust-lang/rust/issues/27731> for more information
    = help: add `#![feature(stdsimd)]` to the crate attributes to enable

error[E0658]: use of unstable library feature 'stdsimd'
   --> /checkout/library/stdarch/crates/std_detect/src/detect/macros.rs:21:25
    |
17  |  /         macro_rules! $macro_name {
19  |  |                 ($feature_lit) => {
19  |  |                 ($feature_lit) => {
20  |  |                     cfg!(target_feature = $feature_lit) ||
21  | /|                         $crate::detect::__is_feature_detected::$feature()
23  | ||             )*
24  | ||             $(
...   ||
47  | ||             };
47  | ||             };
48  | ||         }
    | ||_________- in this expansion of `is_x86_feature_detected!`
127 | |                  }
128 | |              }
    | |_____________________________________________________________________^
    |
    |
   ::: /cargo/registry/src/github.com-1ecc6299db9ec823/memchr-2.4.1/src/memmem/prefilter/mod.rs:286:16
    |
286 |                if is_x86_feature_detected!("avx2") {
    |
    = note: see issue #27731 <https://github.com/rust-lang/rust/issues/27731> for more information
    = help: add `#![feature(stdsimd)]` to the crate attributes to enable


error[E0658]: use of unstable library feature 'stdsimd'
   --> /checkout/library/stdarch/crates/std_detect/src/detect/macros.rs:21:25
    |
17  |  /         macro_rules! $macro_name {
19  |  |                 ($feature_lit) => {
19  |  |                 ($feature_lit) => {
20  |  |                     cfg!(target_feature = $feature_lit) ||
21  | /|                         $crate::detect::__is_feature_detected::$feature()
23  | ||             )*
24  | ||             $(
...   ||
47  | ||             };
47  | ||             };
48  | ||         }
    | ||_________- in this expansion of `is_x86_feature_detected!`
127 | |                  }
128 | |              }
    | |_____________________________________________________________________^
    |
    |
   ::: /cargo/registry/src/github.com-1ecc6299db9ec823/memchr-2.4.1/src/memmem/x86/avx.rs:24:46
    |
24  |                if !cfg!(memchr_runtime_avx) || !is_x86_feature_detected!("avx2") {
    |
    = note: see issue #27731 <https://github.com/rust-lang/rust/issues/27731> for more information
    = help: add `#![feature(stdsimd)]` to the crate attributes to enable


error[E0658]: use of unstable library feature 'stdsimd'
   --> /checkout/library/stdarch/crates/std_detect/src/detect/macros.rs:21:25
    |
17  |  /         macro_rules! $macro_name {
19  |  |                 ($feature_lit) => {
19  |  |                 ($feature_lit) => {
20  |  |                     cfg!(target_feature = $feature_lit) ||
21  | /|                         $crate::detect::__is_feature_detected::$feature()
23  | ||             )*
24  | ||             $(
...   ||
47  | ||             };
47  | ||             };
48  | ||         }
    | ||_________- in this expansion of `is_x86_feature_detected!`
99  | |          #[allow(non_camel_case_types)]
100 | |          #[derive(Copy, Clone)]
    | |__________________________________________________________________________^
    |
    |
   ::: /cargo/registry/src/github.com-1ecc6299db9ec823/crc32fast-1.2.0/src/specialized/pclmulqdq.rs:28:12
    |
28  |            if is_x86_feature_detected!("pclmulqdq")
    |
    = note: see issue #27731 <https://github.com/rust-lang/rust/issues/27731> for more information
    = help: add `#![feature(stdsimd)]` to the crate attributes to enable


error[E0658]: use of unstable library feature 'stdsimd'
   --> /checkout/library/stdarch/crates/std_detect/src/detect/macros.rs:21:25
    |
17  |  /         macro_rules! $macro_name {
19  |  |                 ($feature_lit) => {
19  |  |                 ($feature_lit) => {
20  |  |                     cfg!(target_feature = $feature_lit) ||
21  | /|                         $crate::detect::__is_feature_detected::$feature()
23  | ||             )*
24  | ||             $(
...   ||
47  | ||             };
47  | ||             };
48  | ||         }
    | ||_________- in this expansion of `is_x86_feature_detected!`
111 | |              _last
112 | |          }
    | |_____________________________________________________________________^
    |
    |
   ::: /cargo/registry/src/github.com-1ecc6299db9ec823/crc32fast-1.2.0/src/specialized/pclmulqdq.rs:29:16
    |
29  |                && is_x86_feature_detected!("sse2")
    |
    = note: see issue #27731 <https://github.com/rust-lang/rust/issues/27731> for more information
    = help: add `#![feature(stdsimd)]` to the crate attributes to enable


error[E0658]: use of unstable library feature 'stdsimd'
   --> /checkout/library/stdarch/crates/std_detect/src/detect/macros.rs:21:25
    |
17  |  /         macro_rules! $macro_name {
19  |  |                 ($feature_lit) => {
19  |  |                 ($feature_lit) => {
20  |  |                     cfg!(target_feature = $feature_lit) ||
21  | /|                         $crate::detect::__is_feature_detected::$feature()
23  | ||             )*
24  | ||             $(
...   ||
47  | ||             };
47  | ||             };
48  | ||         }
    | ||_________- in this expansion of `is_x86_feature_detected!`
117 | |                  match self {
117 | |                  match self {
118 | |                      $(Feature::$feature => $feature_lit,)*
    |
    |
   ::: /cargo/registry/src/github.com-1ecc6299db9ec823/crc32fast-1.2.0/src/specialized/pclmulqdq.rs:30:16
    |
30  |                && is_x86_feature_detected!("sse4.1")
    |
    = note: see issue #27731 <https://github.com/rust-lang/rust/issues/27731> for more information
    = help: add `#![feature(stdsimd)]` to the crate attributes to enable


For more information about this error, try `rustc --explain E0658`.
error: could not compile `crc32fast` due to 3 previous errors
warning: build failed, waiting for other jobs to finish...
error[E0658]: use of unstable library feature 'stdsimd'
   --> /checkout/library/stdarch/crates/std_detect/src/detect/macros.rs:21:25
    |
17  |  /         macro_rules! $macro_name {
19  |  |                 ($feature_lit) => {
19  |  |                 ($feature_lit) => {
20  |  |                     cfg!(target_feature = $feature_lit) ||
21  | /|                         $crate::detect::__is_feature_detected::$feature()
23  | ||             )*
24  | ||             $(
...   ||
47  | ||             };
47  | ||             };
48  | ||         }
    | ||_________- in this expansion of `is_x86_feature_detected!`
...   |
119 | |                      Feature::_last => unreachable!(),
    | |_______________________________________________________________________^
    |
   ::: /cargo/registry/src/github.com-1ecc6299db9ec823/snap-1.0.1/src/crc32.rs:28:30
    |
    |
28  |            CheckSummer { sse42: is_x86_feature_detected!("sse4.2") }
    |
    = note: see issue #27731 <https://github.com/rust-lang/rust/issues/27731> for more information
    = help: add `#![feature(stdsimd)]` to the crate attributes to enable

