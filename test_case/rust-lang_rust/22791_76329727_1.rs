 rust
<anon>:14:5: 14:14 error: the requirement `for<'c> _ : 'c` is not satisfied [E0280]
<anon>:14     something(foo_as_bar);
              ^~~~~~~~~
<anon>:14:5: 14:14 error: type mismatch resolving `for<'c> <fn(&'d Foo) -> &'d Bar {foo_as_bar} as core::ops::Fn<(&'c Foo,)>>::Output == _`:
 expected bound lifetime parameter 'c,
    found concrete lifetime [E0271]
<anon>:14     something(foo_as_bar);
              ^~~~~~~~~
<anon>:14:5: 14:14 note: required by `something`
<anon>:14     something(foo_as_bar);
              ^~~~~~~~~
