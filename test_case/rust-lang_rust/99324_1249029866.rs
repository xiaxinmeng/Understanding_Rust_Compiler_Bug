plain
running 7 tests
iiiiFFF
failures:

---- src/lib.rs - LoweringContext::lower_opaque_impl_trait (line 1424) stdout ----
error: expected `;`, found `}`
  |
  |
3 | type TestReturn<'a, T, 'x> = impl Debug + 'x
  |                                             ^ help: add `;` here
4 | } _doctest_main_src_lib_rs_1424_0() }
  | - unexpected token
error: lifetime parameters must be declared prior to type and const parameters
 --> src/lib.rs:1425:24
  |
  |
3 | type TestReturn<'a, T, 'x> = impl Debug + 'x
  |                --------^^- help: reorder the parameters: lifetimes, then consts and types: `<'a, 'x, T>`
error[E0404]: expected trait, found derive macro `Debug`
 --> src/lib.rs:1425:35
  |
  |
3 | type TestReturn<'a, T, 'x> = impl Debug + 'x
  |
help: consider importing one of these items instead
  |
2 | use core::fmt::Debug;
2 | use core::fmt::Debug;
  |
2 | use std::fmt::Debug;
  |

error[E0658]: `impl Trait` in type aliases is unstable
 --> src/lib.rs:1425:30
  |
3 | type TestReturn<'a, T, 'x> = impl Debug + 'x
  |
  = note: see issue #63063 <https://github.com/rust-lang/rust/issues/63063> for more information
  = help: add `#![feature(type_alias_impl_trait)]` to the crate attributes to enable


error: unconstrained opaque type
 --> src/lib.rs:1425:30
  |
3 | type TestReturn<'a, T, 'x> = impl Debug + 'x
  |
  |
  = note: `TestReturn` must be used in combination with a concrete type within the same module
error: aborting due to 5 previous errors

Some errors have detailed explanations: E0404, E0658.
For more information about an error, try `rustc --explain E0404`.
For more information about an error, try `rustc --explain E0404`.
Couldn't compile the test.
---- src/lib.rs - LoweringContext::lower_opaque_impl_trait (line 1416) stdout ----
error[E0404]: expected trait, found derive macro `Debug`
  |
  |
3 | fn test<'a, T: Debug>(x: &'a T) -> impl Debug + 'a {
  |
help: consider importing one of these items instead
  |
2 | use core::fmt::Debug;
2 | use core::fmt::Debug;
  |
2 | use std::fmt::Debug;
  |

error[E0404]: expected trait, found derive macro `Debug`
 --> src/lib.rs:1417:41
  |
3 | fn test<'a, T: Debug>(x: &'a T) -> impl Debug + 'a {
  |
help: consider importing one of these items instead
  |
2 | use core::fmt::Debug;
---
error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0404`.
Couldn't compile the test.
---- src/lib.rs - LoweringContext::lower_opaque_impl_trait (line 1430) stdout ----
error: expected one of `!`, `+`, `::`, `where`, or `{`, found `}`
  |
  |
3 | fn test<'a, T: Debug>(x: &'a T) -> TestReturn<'static, T, 'a>
  |    ---- while parsing this `fn`                              - expected one of `!`, `+`, `::`, `where`, or `{`
4 | } _doctest_main_src_lib_rs_1430_0() }

error: free function without a body
 --> src/lib.rs:1431:1
  |
  |
3 | fn test<'a, T: Debug>(x: &'a T) -> TestReturn<'static, T, 'a>
  | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^- help: provide a definition for the function: `{ <body> }`
error[E0404]: expected trait, found derive macro `Debug`
 --> src/lib.rs:1431:16
  |
  |
3 | fn test<'a, T: Debug>(x: &'a T) -> TestReturn<'static, T, 'a>
  |
help: consider importing one of these items instead
  |
2 | use core::fmt::Debug;
2 | use core::fmt::Debug;
  |
2 | use std::fmt::Debug;
  |

error[E0412]: cannot find type `TestReturn` in this scope
 --> src/lib.rs:1431:36
  |
3 | fn test<'a, T: Debug>(x: &'a T) -> TestReturn<'static, T, 'a>

error: aborting due to 4 previous errors

Some errors have detailed explanations: E0404, E0412.
