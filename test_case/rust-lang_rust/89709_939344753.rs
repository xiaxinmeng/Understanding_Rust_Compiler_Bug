plain
    Checking rustc_data_structures v0.0.0 (/checkout/compiler/rustc_data_structures)
error: lifetime may not live long enough
 --> compiler/rustc_data_structures/src/vec_linked_list.rs:3:62
  |
3 | pub fn iter<Ls>(first: Option<Ls::LinkIndex>, links: &Ls) -> impl Iterator<Item = Ls::LinkIndex>
  |                                                      -       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ opaque type requires that `'1` must outlive `'static`
  |                                                      |
  |                                                      let's call the lifetime of this reference `'1`
  |
help: to allow this `impl Trait` to capture borrowed data with lifetime `'1`, add `'_` as a bound
  |
3 | pub fn iter<Ls>(first: Option<Ls::LinkIndex>, links: &Ls) -> impl Iterator<Item = Ls::LinkIndex> + '_

error[E0614]: type `bool` cannot be dereferenced
  --> compiler/rustc_data_structures/src/steal.rs:37:12
   |
   |
37 |         if *borrow.is_none() {

For more information about this error, try `rustc --explain E0614`.
error: could not compile `rustc_data_structures` due to 2 previous errors
warning: build failed, waiting for other jobs to finish...
