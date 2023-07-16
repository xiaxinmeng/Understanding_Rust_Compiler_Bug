plain
    Checking rustc_query_impl v0.0.0 (/checkout/compiler/rustc_query_impl)
    Checking rustc_monomorphize v0.0.0 (/checkout/compiler/rustc_monomorphize)
    Checking rustc_passes v0.0.0 (/checkout/compiler/rustc_passes)
    Checking rustc_save_analysis v0.0.0 (/checkout/compiler/rustc_save_analysis)
error: 1 positional argument in format string, but no arguments were given
    ::: compiler/rustc_query_impl/src/lib.rs:54:1
     |
54   |    rustc_query_append! { [define_queries!][<'tcx>] }
     |    ------------------------------------------------- in this macro invocation (#1)
     |    ------------------------------------------------- in this macro invocation (#1)
     |
    ::: compiler/rustc_query_impl/src/plumbing.rs:252:1
     |
252  | /  macro_rules! define_queries {
253  | |      (<$tcx:tt>
254  | |       $($(#[$attr:meta])*
255  | |          [$($modifiers:tt)*] fn $name:ident($($K:tt)*) -> $V:ty,)*) => {
...    |
325  | |              rustc_query_description! { $name<$tcx> }
...    |
468  | |      }
469  | |  }
469  | |  }
     | |__- in this expansion of `define_queries!` (#2)
     |
18   |  / rustc_queries! {
19   |  |     query trigger_delay_span_bug(key: DefId) -> () {
20   |  |         desc { "trigger a delay span bug" }
20   |  |         desc { "trigger a delay span bug" }
21   |  |     }
...     |
1603 |  |         desc { "looking up lifetime for trait-object types inside `{:?}`" }
...     |
2066 |  |     }
2067 |  | }
     |  | -
