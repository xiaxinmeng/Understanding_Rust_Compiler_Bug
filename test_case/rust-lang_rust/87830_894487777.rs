plain
   Compiling tracing-attributes v0.1.13
   Compiling rustc_macros v0.1.0 (/checkout/compiler/rustc_macros)
   Compiling chalk-derive v0.55.0
   Compiling chalk-ir v0.55.0
error: this code is interpreted as a block expression, not an array
    |
    |
199 |   #[derive(Clone, Debug, PartialEq, Eq, Hash, Fold, Visit)]
    |                                               ^^^^ in this derive macro expansion
   ::: /cargo/registry/src/github.com-1ecc6299db9ec823/synstructure-0.12.4/src/macros.rs:94:9
    |
94  | /         pub fn $derives(
94  | /         pub fn $derives(
95  | |             i: $crate::macros::TokenStream
96  | |         ) -> $crate::macros::TokenStream {
    | |________________________________________- in this expansion of `#[derive(Fold)]`
    |
    = note: to define an array, one would use square brackets instead of curly braces
help: try using [] instead of {}
    |
199 | #[derive(Clone, Debug, PartialEq, Eq, Hash, [], Visit)]

error: proc-macro derive produced unparseable tokens
   --> /cargo/registry/src/github.com-1ecc6299db9ec823/chalk-ir-0.55.0/src/lib.rs:199:45
    |
    |
199 | #[derive(Clone, Debug, PartialEq, Eq, Hash, Fold, Visit)]


error: this code is interpreted as a block expression, not an array
    |
    |
199 |   #[derive(Clone, Debug, PartialEq, Eq, Hash, Fold, Visit)]
    |                                                     ^^^^^ in this derive macro expansion
   ::: /cargo/registry/src/github.com-1ecc6299db9ec823/synstructure-0.12.4/src/macros.rs:94:9
    |
94  | /         pub fn $derives(
94  | /         pub fn $derives(
95  | |             i: $crate::macros::TokenStream
96  | |         ) -> $crate::macros::TokenStream {
    | |________________________________________- in this expansion of `#[derive(Visit)]`
    |
    = note: to define an array, one would use square brackets instead of curly braces
help: try using [] instead of {}
    |
199 | #[derive(Clone, Debug, PartialEq, Eq, Hash, Fold, [])]

error: proc-macro derive produced unparseable tokens
   --> /cargo/registry/src/github.com-1ecc6299db9ec823/chalk-ir-0.55.0/src/lib.rs:199:51
    |
    |
199 | #[derive(Clone, Debug, PartialEq, Eq, Hash, Fold, Visit)]

   Compiling tracing v0.1.25
   Compiling rustc_index v0.0.0 (/checkout/compiler/rustc_index)
   Compiling rustc_index v0.0.0 (/checkout/compiler/rustc_index)
error: this code is interpreted as a block expression, not an array
   --> compiler/rustc_index/src/bit_set.rs:740:32
    |
740 |   #[derive(Clone, Eq, PartialEq, Decodable, Encodable)]
    |                                  ^^^^^^^^^ in this derive macro expansion
   ::: /cargo/registry/src/github.com-1ecc6299db9ec823/synstructure-0.12.4/src/macros.rs:94:9
    |
94  | /         pub fn $derives(
94  | /         pub fn $derives(
95  | |             i: $crate::macros::TokenStream
96  | |         ) -> $crate::macros::TokenStream {
    | |________________________________________- in this expansion of `#[derive(Decodable)]`
    |
    = note: to define an array, one would use square brackets instead of curly braces
help: try using [] instead of {}
    |
740 | #[derive(Clone, Eq, PartialEq, [], Encodable)]

error: proc-macro derive produced unparseable tokens
   --> compiler/rustc_index/src/bit_set.rs:740:32
    |
    |
740 | #[derive(Clone, Eq, PartialEq, Decodable, Encodable)]


error: this code is interpreted as a block expression, not an array
   --> compiler/rustc_index/src/bit_set.rs:740:43
    |
740 |   #[derive(Clone, Eq, PartialEq, Decodable, Encodable)]
    |                                             ^^^^^^^^^ in this derive macro expansion
   ::: /cargo/registry/src/github.com-1ecc6299db9ec823/synstructure-0.12.4/src/macros.rs:94:9
    |
94  | /         pub fn $derives(
94  | /         pub fn $derives(
95  | |             i: $crate::macros::TokenStream
96  | |         ) -> $crate::macros::TokenStream {
    | |________________________________________- in this expansion of `#[derive(Encodable)]`
    |
    = note: to define an array, one would use square brackets instead of curly braces
help: try using [] instead of {}
    |
740 | #[derive(Clone, Eq, PartialEq, Decodable, [])]

error: proc-macro derive produced unparseable tokens
   --> compiler/rustc_index/src/bit_set.rs:740:43
    |
    |
740 | #[derive(Clone, Eq, PartialEq, Decodable, Encodable)]

error: could not compile `rustc_index` due to 4 previous errors
warning: build failed, waiting for other jobs to finish...
error: build failed
