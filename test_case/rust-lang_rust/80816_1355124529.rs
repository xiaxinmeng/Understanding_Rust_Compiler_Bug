
error[[E0283]](https://doc.rust-lang.org/nightly/error-index.html#E0283): type annotations needed
 --> src/lib.rs:7:48
  |
7 |     let guard: arc_swap::Guard<Arc<usize>> = s.load();
  |                                                ^^^^
  |
  = note: multiple `impl`s satisfying `ArcSwapAny<Arc<usize>>: arc_swap::access::Access<_>` found in the `arc_swap` crate:
          - impl<T, S> arc_swap::access::Access<T> for ArcSwapAny<Arc<T>, S>
            where S: Strategy<Arc<T>>;
          - impl<T, S> arc_swap::access::Access<T> for ArcSwapAny<T, S>
            where T: RefCnt, S: Strategy<T>;
  = note: required for `Arc<ArcSwapAny<Arc<usize>>>` to implement `arc_swap::access::Access<_>`
help: try using a fully qualified path to specify the expected types
  |
7 |     let guard: arc_swap::Guard<Arc<usize>> = <Arc<ArcSwapAny<Arc<usize>>> as arc_swap::access::Access<T>>::load(&s);
  |                                              ++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++ ~

For more information about this error, try `rustc --explain E0283`.
