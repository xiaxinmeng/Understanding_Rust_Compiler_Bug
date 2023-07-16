plain
   Compiling rustc_trait_selection v0.0.0 (/checkout/compiler/rustc_trait_selection)
error[E0061]: this function takes 1 argument but 0 arguments were supplied
   --> compiler/rustc_trait_selection/src/traits/fulfill.rs:675:33
    |
675 |         if obligation.predicate.is_global() {
    |                                 |
    |                                 expected 1 argument
    |
note: associated function defined here
