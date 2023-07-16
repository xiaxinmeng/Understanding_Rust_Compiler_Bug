plain
   Compiling memchr v2.4.1
   Compiling std v0.0.0 (/checkout/library/std)
   Compiling compiler_builtins v0.1.49
   Compiling unwind v0.0.0 (/checkout/library/unwind)
error[E0277]: the trait bound `<P as pattern::Pattern<'_>>::Searcher: Clone` is not satisfied
     |
1467 |   #[derive(Clone)]
     |            ----- in this derive macro expansion
     |            ----- in this derive macro expansion
1468 |   pub struct SplitRInclusive<'a, P: Pattern<'a>>(pub(super) SplitInternal<'a, P>);
     |                                                  ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the trait `Clone` is not implemented for `<P as pattern::Pattern<'_>>::Searcher`
    ::: library/core/src/clone.rs:140:1
     |
     |
140  | / pub macro Clone($item:item) {
142  | | }
142  | | }
     | |_- in this expansion of `#[derive(Clone)]`
     |
     = note: required because of the requirements on the impl of `Clone` for `SplitInternal<'_, P>`
note: required by `Clone::clone`
     |
122  |     fn clone(&self) -> Self;
     |     ^^^^^^^^^^^^^^^^^^^^^^^^

