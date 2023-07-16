
error[E0277]: the trait bound `rustc_data_structures::sorted_map::SortedIndexMultiMap<u32, rustc_span::Symbol, &Attribute>: std::marker::Copy` is not satisfied
    --> compiler/rustc_middle/src/ty/query.rs:255:91
     |
178  | /  macro_rules! define_callbacks {
179  | |      (
180  | |       $($(#[$attr:meta])*
181  | |          [$($modifiers:tt)*] fn $name:ident($($K:tt)*) -> $V:ty,)*) => {
...    |
255  | |                  let cached = try_get_cached(self.tcx, &self.tcx.query_caches.$name, &key, copy);
     | |                                                                                            ^^^^ the trait `std::marker::Copy` is not implemented for `rustc_data_structures::sorted_map::SortedIndexMultiMap<u32, rustc_span::Symbol, &Attribute>`
...    |
324  | |      };
325  | |  }
     | |__- in this expansion of `define_callbacks!` (#2)
...
339  |    rustc_query_append! { define_callbacks! }
     |    ----------------------------------------- in this macro invocation (#1)
     |
    ::: compiler/rustc_middle/src/query/mod.rs:18:1
     |
18   |  / rustc_queries! {
19   |  |     query trigger_delay_span_bug(key: DefId) -> () {
20   |  |         desc { "trigger a delay span bug" }
21   |  |     }
...     |
2101 |  |     }
2102 |  | }
     |  | -
     |  | |
     |  |_in this expansion of `rustc_query_append!` (#1)
     |    in this macro invocation (#2)
     |
note: required by a bound in `ty::query::copy`
    --> compiler/rustc_middle/src/ty/query.rs:114:12
     |
114  | fn copy<T: Copy>(x: &T) -> T {
     |            ^^^^ required by this bound in `ty::query::copy`
