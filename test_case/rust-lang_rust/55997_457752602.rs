
error[E01234]: Multiple contradictory defining uses of `Foo` were found:

1. src/lib.rs:x:y | existential type Foo: Debug;

    defines `Foo` as `std::fmt::Debug`

2. src/lib.rs:x:y | assert_send(foo());

    defines `Foo` as `Send` because of the definition of `assert_send`

All uses of `Foo` in the module where it's defined and its submodules must agree on the bounds.
