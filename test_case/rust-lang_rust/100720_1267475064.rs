plain
   ::: compiler/rustc_query_impl/src/plumbing.rs:444:1
    |
444 | /  macro_rules! define_queries {
445 | |      (
446 | |       $($(#[$attr:meta])*
447 | |          [$($modifiers:tt)*] fn $name:ident($($K:tt)*) -> $V:ty,)*) => {
469 | |              rustc_query_description! { $name }
    | |              ---------------------------------- in this macro invocation (#3)
...   |
631 | |      }
631 | |      }
632 | |  }
    | |__- in this expansion of `define_queries!` (#2)
    |
   ::: compiler/rustc_query_impl/src/lib.rs:56:1
    |
56  |    rustc_query_append! { define_queries! }
    |
    |
    = note: did you intend to capture a variable `key` from the surrounding scope?
    = note: to avoid ambiguity, `format_args!` cannot capture variables when the format string is expanded from a macro
   Compiling rustc_resolve v0.0.0 (/checkout/compiler/rustc_resolve)
[RUSTC-TIMING] rustc_query_impl test:false 3.859
error: could not compile `rustc_query_impl` due to previous error
warning: build failed, waiting for other jobs to finish...
