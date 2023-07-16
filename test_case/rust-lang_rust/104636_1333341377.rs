plain
     |                 ----^^^^^^^^^^^^^^
     |                 |
     |                 help: remove this `mut`
     |
     = note: `-D unused-mut` implied by `-D warnings`
error[E0382]: use of moved value: `regular_traits`
    --> compiler/rustc_hir_analysis/src/astconv/mod.rs:1330:17
     |
1319 |             let mut regular_traits = regular_traits
1319 |             let mut regular_traits = regular_traits
     |                 ------------------ move occurs because `regular_traits` has type `std::iter::Map<std::slice::Iter<'_, rustc_trait_selection::traits::util::TraitAliasExpansionInfo<'_>>, [closure@compiler/rustc_hir_analysis/src/astconv/mod.rs:1321:22: 1321:25]>`, which does not implement the `Copy` trait
...
1323 |                 .filter_map(|name| name.strip_prefix("NewTrait")?.parse::<u32>().ok())
     |                  --------------------------------------------------------------------- `regular_traits` moved due to this method call
1330 |                 regular_traits
     |                 ^^^^^^^^^^^^^^ value used here after move
     |
     |
note: this function takes ownership of the receiver `self`, which moves `regular_traits`
     |
     |
941  |     fn filter_map<B, F>(self, f: F) -> FilterMap<Self, F>

For more information about this error, try `rustc --explain E0382`.
error: could not compile `rustc_hir_analysis` due to 2 previous errors
warning: build failed, waiting for other jobs to finish...
