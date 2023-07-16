rust
error[E0643]: method `extend` has incompatible signature for trait
   --> xcrate-spans.rs:3:29
    |
3   |     fn extend(&mut self, _: impl IntoIterator<Item = ()>) {}
    |                             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected generic parameter, found `impl Trait`
    |
   ::: /rustc/1f57e4841157d5cbd4c4e22018f93bd1801c98c2/src/libcore/iter/traits.rs:355:15
    |
355 |     fn extend<T: IntoIterator<Item=A>>(&mut self, iter: T);
    |               - declaration in trait here
