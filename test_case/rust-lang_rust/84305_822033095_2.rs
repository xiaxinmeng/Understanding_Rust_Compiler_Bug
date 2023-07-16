
warning: unsatisfied lifetime constraints
  --> src/main.rs:14:19
   |
13 | fn extend_lifetime<'a, 'b, T: ?Sized>(x: &'a T) -> &'b T {
   |                    -- lifetime `'a` defined here
14 |     Foo::fun(&|_| impl_foo(x))(PhantomData)
   |                   ^^^^^^^^^^^ returning this value requires that `'a` must outlive `'static`
   |
   = warning: this error has been downgraded to a warning for backwards compatibility with previous releases
   = warning: this represents potential undefined behavior in your code and this warning will become a hard error in the future

    Finished dev [unoptimized + debuginfo] target(s) in 0.83s
