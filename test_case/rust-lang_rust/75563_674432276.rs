
    Checking rustc_builtin_macros v0.0.0 (/checkout/src/librustc_builtin_macros)
error[E0107]: wrong number of type arguments: expected 1, found 0
    --> src/librustc_middle/mir/mod.rs:1510:51
     |
1510 |   #[derive(Clone, Debug, PartialEq, RustcEncodable, RustcDecodable, HashStable, TypeFoldable)]
     |                                                     ^^^^^^^^^^^^^^
     |                                                     |
     |                                                     expected 1 type argument
     |                                                     in this macro invocation
     | 
    ::: /checkout/library/core/src/macros/mod.rs:1432:5
     |
1432 | /     pub macro RustcDecodable($item:item) {
1433 | |         /* compiler built-in */
1434 | |     }
     | |_____- in this expansion of `#[derive(RustcDecodable)]`

error[E0107]: wrong number of type arguments: expected 1, found 0
    --> src/librustc_middle/mir/mod.rs:1510:35
     |
1510 |   #[derive(Clone, Debug, PartialEq, RustcEncodable, RustcDecodable, HashStable, TypeFoldable)]
     |                                     ^^^^^^^^^^^^^^
     |                                     |
     |                                     expected 1 type argument
