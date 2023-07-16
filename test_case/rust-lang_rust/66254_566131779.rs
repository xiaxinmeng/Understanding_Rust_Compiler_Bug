rust
2019-12-16T16:14:41.5282737Z error: stable const functions must have either `rustc_const_stable` or `rustc_const_unstable` attribute
2019-12-16T16:14:41.5283245Z --> src/libcore/alloc.rs:122:5
2019-12-16T16:14:41.5283508Z |
2019-12-16T16:14:41.5283816Z 122 | / pub const fn new<T>() -> Self {
2019-12-16T16:14:41.5284154Z 123 | | let (size, align) = size_align::<T>();
2019-12-16T16:14:41.5284518Z 124 | | // Note that the align is guaranteed by rustc to be a power of two and
2019-12-16T16:14:41.5284891Z 125 | | // the size+align combo is guaranteed to fit in our address space. As a
2019-12-16T16:14:41.5285145Z ... |
2019-12-16T16:14:41.5285430Z 130 | | }
2019-12-16T16:14:41.5285738Z 131 | | }
2019-12-16T16:14:41.5285978Z | |_____^
