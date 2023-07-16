plain
    Checking rustc_codegen_llvm v0.0.0 (/checkout/compiler/rustc_codegen_llvm)
error[E0282]: type annotations needed
   --> compiler/rustc_trait_selection/src/traits/mod.rs:554:42
    |
552 |                 let inner_most_trait_ref = *inner_most_trait_ref;
    |                     -------------------- consider giving `inner_most_trait_ref` a type
553 |                 let mut direct_super_traits_iter = tcx
554 |                     .super_predicates_of(inner_most_trait_ref.def_id())
    |
    = note: type must be known at this point


error[E0282]: type annotations needed for `SmallVec<A>`
    |
156 | / macro_rules! smallvec {
156 | / macro_rules! smallvec {
157 | |     // count helper: transform any expression into 1
158 | |     (@one $x:expr) => (1usize);
159 | |     ($elem:expr; $n:expr) => ({
165 | |         let mut vec = $crate::SmallVec::new();
    | |                       ^^^^^^^^^^^^^^^^^^^^^ cannot infer type for type parameter `A`
...   |
172 | |     });
172 | |     });
173 | | }
    | |_- in this expansion of `smallvec!`
    | 
   ::: compiler/rustc_trait_selection/src/traits/mod.rs:622:9
    |
622 |       let mut entries = smallvec![];
    |           -----------   ----------- in this macro invocation
    |           |
    |           consider giving `entries` the explicit type `SmallVec<A>`, where the type parameter `A` is specified
error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0282`.
error: could not compile `rustc_trait_selection`
