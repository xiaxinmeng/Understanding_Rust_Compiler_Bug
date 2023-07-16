plain
   --> compiler/rustc_borrowck/src/diagnostics/conflict_errors.rs:743:17
    |
741 |             .type_implements_trait(
    |              --------------------- required by a bound introduced by this call
742 |                 tcx.lang_items().clone_trait().unwrap(),
743 |                 tcx.erase_regions(ty),
    |
    = help: the trait `Iterator` is not implemented for `rustc_middle::ty::Ty<'tcx>`
    = note: required for `rustc_middle::ty::Ty<'tcx>` to implement `IntoIterator`
note: required by a bound in `type_implements_trait`
note: required by a bound in `type_implements_trait`
   --> /checkout/compiler/rustc_trait_selection/src/infer.rs:55:22
    |
55  |         params: impl IntoIterator<Item = impl Into<GenericArg<'tcx>>>,

error[E0061]: this function takes 3 arguments but 4 arguments were supplied
   --> compiler/rustc_borrowck/src/diagnostics/conflict_errors.rs:741:14
    |
    |
741 |             .type_implements_trait(
    |              ^^^^^^^^^^^^^^^^^^^^^
...
744 |                 ty::List::empty(),
    |                 ----------------- argument of type `&List<_>` unexpected
note: associated function defined here
   --> /checkout/compiler/rustc_trait_selection/src/infer.rs:52:8
    |
52  |     fn type_implements_trait(
52  |     fn type_implements_trait(
    |        ^^^^^^^^^^^^^^^^^^^^^
help: remove the extra argument
    |
741 |             .type_implements_trait(tcx.lang_items().clone_trait().unwrap(), tcx.erase_regions(ty), self.param_env)

Some errors have detailed explanations: E0061, E0277.
For more information about an error, try `rustc --explain E0061`.
error: could not compile `rustc_borrowck` due to 2 previous errors
