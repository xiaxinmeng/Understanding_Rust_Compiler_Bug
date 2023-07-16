plain
   Compiling memchr v2.4.1
   Compiling std v0.0.0 (/checkout/library/std)
   Compiling compiler_builtins v0.1.49
   Compiling unwind v0.0.0 (/checkout/library/unwind)
error: `[u32; 2]` is forbidden as the type of a const generic parameter
   --> library/core/src/../../portable-simd/crates/core_simd/src/permute.rs:30:39
4   | / macro_rules! impl_shuffle_lane {
4   | / macro_rules! impl_shuffle_lane {
5   | |     { $fn:ident, $n:literal } => {
6   | |         impl<T> Simd<T, $n>
7   | |         where
...   |
30  | |             pub fn shuffle<const IDX: [u32; $n]>(self, second: Self) -> Self {
...   |
147 | |     }
148 | | }
148 | | }
    | |_- in this expansion of `impl_shuffle_lane!`
149 | 
150 |   impl_shuffle_lane! { simd_shuffle2, 2 }
    |
    |
    = note: the only supported types are integers, `bool` and `char`
    = help: more complex types are supported with `#![feature(adt_const_params)]`

error: `[u32; 4]` is forbidden as the type of a const generic parameter
   --> library/core/src/../../portable-simd/crates/core_simd/src/permute.rs:30:39
4   | / macro_rules! impl_shuffle_lane {
4   | / macro_rules! impl_shuffle_lane {
5   | |     { $fn:ident, $n:literal } => {
6   | |         impl<T> Simd<T, $n>
7   | |         where
...   |
30  | |             pub fn shuffle<const IDX: [u32; $n]>(self, second: Self) -> Self {
...   |
147 | |     }
148 | | }
148 | | }
    | |_- in this expansion of `impl_shuffle_lane!`
...
151 |   impl_shuffle_lane! { simd_shuffle4, 4 }
    |
    |
    = note: the only supported types are integers, `bool` and `char`
    = help: more complex types are supported with `#![feature(adt_const_params)]`

error: `[u32; 8]` is forbidden as the type of a const generic parameter
   --> library/core/src/../../portable-simd/crates/core_simd/src/permute.rs:30:39
4   | / macro_rules! impl_shuffle_lane {
4   | / macro_rules! impl_shuffle_lane {
5   | |     { $fn:ident, $n:literal } => {
6   | |         impl<T> Simd<T, $n>
7   | |         where
...   |
30  | |             pub fn shuffle<const IDX: [u32; $n]>(self, second: Self) -> Self {
...   |
147 | |     }
148 | | }
148 | | }
    | |_- in this expansion of `impl_shuffle_lane!`
...
152 |   impl_shuffle_lane! { simd_shuffle8, 8 }
    |
    |
    = note: the only supported types are integers, `bool` and `char`
    = help: more complex types are supported with `#![feature(adt_const_params)]`

error: `[u32; 16]` is forbidden as the type of a const generic parameter
   --> library/core/src/../../portable-simd/crates/core_simd/src/permute.rs:30:39
4   | / macro_rules! impl_shuffle_lane {
4   | / macro_rules! impl_shuffle_lane {
5   | |     { $fn:ident, $n:literal } => {
6   | |         impl<T> Simd<T, $n>
7   | |         where
...   |
30  | |             pub fn shuffle<const IDX: [u32; $n]>(self, second: Self) -> Self {
...   |
147 | |     }
148 | | }
148 | | }
    | |_- in this expansion of `impl_shuffle_lane!`
...
153 |   impl_shuffle_lane! { simd_shuffle16, 16 }
    |
    |
    = note: the only supported types are integers, `bool` and `char`
    = help: more complex types are supported with `#![feature(adt_const_params)]`

error: `[u32; 32]` is forbidden as the type of a const generic parameter
   --> library/core/src/../../portable-simd/crates/core_simd/src/permute.rs:30:39
4   | / macro_rules! impl_shuffle_lane {
4   | / macro_rules! impl_shuffle_lane {
5   | |     { $fn:ident, $n:literal } => {
6   | |         impl<T> Simd<T, $n>
7   | |         where
...   |
30  | |             pub fn shuffle<const IDX: [u32; $n]>(self, second: Self) -> Self {
...   |
147 | |     }
148 | | }
148 | | }
    | |_- in this expansion of `impl_shuffle_lane!`
...
154 |   impl_shuffle_lane! { simd_shuffle32, 32 }
    |
    |
    = note: the only supported types are integers, `bool` and `char`
    = help: more complex types are supported with `#![feature(adt_const_params)]`
error: could not compile `core` due to 5 previous errors
warning: build failed, waiting for other jobs to finish...
error: build failed
Build completed unsuccessfully in 0:00:08
