
$ rg -trust trivial_
src/libcore/iter/range.rs
65:            #[allow(trivial_numeric_casts)]
125:            #[allow(trivial_numeric_casts)]

src/librustc_typeck/check/mod.rs
1448:#[allow(trivial_numeric_casts)]

# Tests

src/test/compile-fail/liveness-unused.rs
13:#![allow(dead_code, non_camel_case_types, trivial_numeric_casts)]

src/test/compile-fail/object-safety-by-value-self.rs
15:#![allow(trivial_casts)]

src/test/run-pass/trivial_casts.rs
13:#![allow(trivial_casts, trivial_numeric_casts)]

src/test/compile-fail/trivial_casts.rs
11:// Test the trivial_casts and trivial_numeric_casts lints. For each error we also
14:#![deny(trivial_casts, trivial_numeric_casts)]
