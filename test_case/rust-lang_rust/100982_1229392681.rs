plain
   Compiling askama_shared v0.12.0
   Compiling askama_derive v0.11.0
   Compiling askama v0.11.0
   Compiling rustdoc v0.0.0 (/checkout/src/librustdoc)
error[E0277]: the trait bound `HashSet<_, BuildHasherDefault<FxHasher>>: std::default::Default` is not satisfied
    |
    |
302 |                     LazyLock::new(FxHashSet::default);
    |                                   ^^^^^^^^^^^^^^^^^^ the trait `~const std::default::Default` is not implemented for `HashSet<_, BuildHasherDefault<FxHasher>>`
    |
note: the trait `std::default::Default` is implemented for `HashSet<_, BuildHasherDefault<FxHasher>>`, but that implementation is not `const`
    |
    |
302 |                     LazyLock::new(FxHashSet::default);

For more information about this error, try `rustc --explain E0277`.
error: could not compile `rustdoc` due to previous error
Build completed unsuccessfully in 0:14:48
