txt
error[E0277]: static assertion failed
  --> tests/messages.rs:14:1
   |
14 | assert_impl_one!(Foo: A, B, C);
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   | |
   | `Foo` must implement exactly one of the given traits
   | required by `_::{{closure}}#0::AmbiguousIfMoreThanOne::some_item`
   |
   = help: the trait `_::{{closure}}#0::AmbiguousIfMoreThanOne<_>` is not implemented for `Foo`
   = note: this error originates in a macro outside of the current crate (in Nightly builds, run with -Z external-macro-backtrace for more info)
