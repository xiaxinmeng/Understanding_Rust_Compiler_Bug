plain
    Checking rustc_builtin_macros v0.0.0 (/checkout/compiler/rustc_builtin_macros)
error: path statement with no effect
    --> compiler/rustc_middle/src/ty/query.rs:171:28
     |
170  |    / macro_rules! opt_remap_env_constness {
171  |    |     ([][$name:ident]) => { $name };
     |    |                            ^^^^^
172  |    |     ([(remap_env_constness) $($rest:tt)*][$name:ident]) => {
173  |    |         $name.without_const()
...       |
176  |    |         opt_remap_env_constness!([$($modifiers)*][$name])
177  |    |     };
178  |    | }
     |    | -
     |    | |
     |    | |
     |    |_in this expansion of `opt_remap_env_constness!` (#3)
     |      in this expansion of `opt_remap_env_constness!` (#4)
387  |    / macro_rules! define_feedable {
387  |    / macro_rules! define_feedable {
388  |    |     ($($(#[$attr:meta])* [$($modifiers:tt)*] fn $name:ident($($K:tt)*) -> $V:ty,)*) => {
389  |    |         $(impl<'tcx, K: IntoQueryParam<$($K)*> + Copy> TyCtxtFeed<'tcx, K> {
390  |    |             $(#[$attr])*
...       |
394  |    |                 opt_remap_env_constness!([$($modifiers)*][key]);
...       |
422  |    |     }
423  |    | }
     |    |_- in this expansion of `define_feedable!` (#2)
     |    |_- in this expansion of `define_feedable!` (#2)
...
438  |      rustc_feedable_queries! { define_feedable! }
     |
    ::: compiler/rustc_middle/src/query/mod.rs:24:1
     |
24   | /    rustc_queries! {
---
2212 | |        }
2213 | |    }
     | |    -
     | |    |
     | |____in this expansion of `rustc_feedable_queries!` (#1)
     |
     |
     = note: `-D path-statements` implied by `-D warnings`
error: path statement with no effect
    --> compiler/rustc_middle/src/ty/query.rs:171:28
     |
170  |     / macro_rules! opt_remap_env_constness {
170  |     / macro_rules! opt_remap_env_constness {
171  |     |     ([][$name:ident]) => { $name };
     |     |                            ^^^^^
172  |     |     ([(remap_env_constness) $($rest:tt)*][$name:ident]) => {
173  |     |         $name.without_const()
...        |
176  |     |         opt_remap_env_constness!([$($modifiers)*][$name])
     |     |         |
     |     |         in this macro invocation (#4)
     |     |         in this macro invocation (#5)
177  |     |     };
177  |     |     };
178  |     | }
     |     | -
     |     | |
     |     | in this expansion of `opt_remap_env_constness!` (#3)
     |     |_in this expansion of `opt_remap_env_constness!` (#4)
     |       in this expansion of `opt_remap_env_constness!` (#5)
387  |     / macro_rules! define_feedable {
387  |     / macro_rules! define_feedable {
388  |     |     ($($(#[$attr:meta])* [$($modifiers:tt)*] fn $name:ident($($K:tt)*) -> $V:ty,)*) => {
389  |     |         $(impl<'tcx, K: IntoQueryParam<$($K)*> + Copy> TyCtxtFeed<'tcx, K> {
390  |     |             $(#[$attr])*
...        |
394  |     |                 opt_remap_env_constness!([$($modifiers)*][key]);
...        |
422  |     |     }
423  |     | }
     |     |_- in this expansion of `define_feedable!` (#2)
     |     |_- in this expansion of `define_feedable!` (#2)
...
438  |       rustc_feedable_queries! { define_feedable! }
     |
    ::: compiler/rustc_middle/src/query/mod.rs:24:1
     |
24   | /     rustc_queries! {
---
2212 | |         }
2213 | |     }
     | |     -
     | |     |
     | |_____in this expansion of `rustc_feedable_queries!` (#1)

error: path statement with no effect
    --> compiler/rustc_middle/src/ty/query.rs:171:28
     |
     |
170  |    / macro_rules! opt_remap_env_constness {
171  |    |     ([][$name:ident]) => { $name };
     |    |                            ^^^^^
172  |    |     ([(remap_env_constness) $($rest:tt)*][$name:ident]) => {
173  |    |         $name.without_const()
...       |
176  |    |         opt_remap_env_constness!([$($modifiers)*][$name])
177  |    |     };
178  |    | }
     |    | -
     |    | |
     |    | |
     |    |_in this expansion of `opt_remap_env_constness!` (#3)
     |      in this expansion of `opt_remap_env_constness!` (#4)
387  |    / macro_rules! define_feedable {
387  |    / macro_rules! define_feedable {
388  |    |     ($($(#[$attr:meta])* [$($modifiers:tt)*] fn $name:ident($($K:tt)*) -> $V:ty,)*) => {
389  |    |         $(impl<'tcx, K: IntoQueryParam<$($K)*> + Copy> TyCtxtFeed<'tcx, K> {
390  |    |             $(#[$attr])*
...       |
394  |    |                 opt_remap_env_constness!([$($modifiers)*][key]);
...       |
422  |    |     }
423  |    | }
     |    |_- in this expansion of `define_feedable!` (#2)
     |    |_- in this expansion of `define_feedable!` (#2)
...
438  |      rustc_feedable_queries! { define_feedable! }
     |
    ::: compiler/rustc_middle/src/query/mod.rs:24:1
     |
24   | /    rustc_queries! {
---
2212 | |        }
2213 | |    }
     | |    -
     | |    |
     | |____in this expansion of `rustc_feedable_queries!` (#1)

error: path statement with no effect
    --> compiler/rustc_middle/src/ty/query.rs:171:28
     |
     |
170  |   / macro_rules! opt_remap_env_constness {
171  |   |     ([][$name:ident]) => { $name };
     |   |                            ^^^^^
172  |   |     ([(remap_env_constness) $($rest:tt)*][$name:ident]) => {
173  |   |         $name.without_const()
177  |   |     };
178  |   | }
178  |   | }
     |   |_- in this expansion of `opt_remap_env_constness!` (#3)
387  |   / macro_rules! define_feedable {
387  |   / macro_rules! define_feedable {
388  |   |     ($($(#[$attr:meta])* [$($modifiers:tt)*] fn $name:ident($($K:tt)*) -> $V:ty,)*) => {
389  |   |         $(impl<'tcx, K: IntoQueryParam<$($K)*> + Copy> TyCtxtFeed<'tcx, K> {
390  |   |             $(#[$attr])*
...      |
394  |   |                 opt_remap_env_constness!([$($modifiers)*][key]);
     |   |                 ----------------------------------------------- in this macro invocation (#3)
422  |   |     }
423  |   | }
     |   |_- in this expansion of `define_feedable!` (#2)
...
...
438  |     rustc_feedable_queries! { define_feedable! }
     |
    ::: compiler/rustc_middle/src/query/mod.rs:24:1
     |
24   | /   rustc_queries! {
24   | /   rustc_queries! {
25   | |       query trigger_delay_span_bug(key: DefId) -> () {
26   | |           desc { "triggering a delay span bug" }
27   | |       }
...    |
2212 | |       }
2213 | |   }
     | |   -
     | |   |
     | |___in this expansion of `rustc_feedable_queries!` (#1)

error: could not compile `rustc_middle` due to 7 previous errors
warning: build failed, waiting for other jobs to finish...
error: could not compile `rustc_middle` due to 7 previous errors
