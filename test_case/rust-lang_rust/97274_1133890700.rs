plain
    Checking core v0.0.0 (/checkout/library/core)
   Compiling libc v0.2.126
   Compiling memchr v2.4.1
   Compiling std v0.0.0 (/checkout/library/std)
error: expected one of `!`, `(`, `+`, `::`, `<`, `where`, or `{`, found doc comment `/// Returns the length of the array.`
    |
    |
396 | impl<T, const N: usize> [T; N] {
    |                                - while parsing this item list starting here
439 |     pub fn len() -> usize
    |                          - expected one of 7 possible tokens
440 |     /// Returns the length of the array.
    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ unexpected token
    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ unexpected token
...
753 | }
    | - the item list ends here
   Compiling compiler_builtins v0.1.71
   Compiling unwind v0.0.0 (/checkout/library/unwind)
   Compiling unwind v0.0.0 (/checkout/library/unwind)
error[E0599]: `[(); N]` is not an iterator
     |
     |
44   |     [(); N].map(|_| {
     |             ^^^ `[(); N]` is not an iterator; try calling `.into_iter()` or `.iter()`
note: the following trait bounds were not satisfied:
note: the following trait bounds were not satisfied:
      `[(); N]: Iterator`
      `[()]: Iterator`
     |
     |
3773 | impl<I: Iterator + ?Sized> Iterator for &mut I {
     |         |
     |         unsatisfied trait bound introduced here
     = help: items from traits can only be used if the trait is implemented and in scope
     = help: items from traits can only be used if the trait is implemented and in scope
note: `Iterator` defines an item `map`, perhaps you need to implement it
     |
62   | pub trait Iterator {
     | ^^^^^^^^^^^^^^^^^^

