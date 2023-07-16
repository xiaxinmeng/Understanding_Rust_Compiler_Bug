rust
trait Foo {
    fn bar(&mut self);
}

impl Foo for fn() {
    fn bar(&mut self) {
        (*self)()
    }
}

fn takes_foo(_: impl Foo) {}

fn is_foo() {}

fn main() {
    takes_foo(is_foo);
    // this gives the error
    //
    //     error[E0277]: the trait bound `fn() {is_foo}: Foo` is not satisfied
    //       --> src/main.rs:16:15
    //        |
    //     18 |     takes_foo(is_foo);
    //        |     --------- ^^^^^^ the trait `Foo` is not implemented for `fn() {is_foo}`
    //        |     |
    //        |     required by a bound introduced by this call
    //        |
    //        = help: the following implementations were found:
    //                  <fn() as Foo>
    //
    // but the help doesn't make it clear that the `{is_foo}` part is relevant,
    // and that this can be fixed with a cast to a FnPtr:
    takes_foo(is_foo as fn());

    takes_foo(|| {});
    // this gives the error
    //
    //     error[E0277]: the trait bound `[closure@src/main.rs:34:15: 34:20]: Foo` is not satisfied
    //       --> src/main.rs:34:5
    //        |
    //     33 |     takes_foo(|_: &()| true);
    //        |     ^^^^^^^^^ the trait `Foo` is not implemented for `[closure@src/main.rs:34:15: 34:20]`
    //
    // which doesn't even mention that the closure can be cast to a FnPtr, which
    // _would_ satisfy the bound:
    takes_foo((|| {}) as fn());
    // (note also the extra () needed to avoid the cast being applied to `{}`)
}
