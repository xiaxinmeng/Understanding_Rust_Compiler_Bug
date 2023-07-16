 rust
<anon>:17:13: 17:14 error: `x` does not live long enough
<anon>:17     FooRef(&x).borrow()
                      ^
<anon>:15:11: 18:2 note: reference must be valid for the destruction scope surrounding block at 15:10...
<anon>:15 fn main() {
<anon>:16     let x = Foo;
<anon>:17     FooRef(&x).borrow()
<anon>:18 }
<anon>:16:16: 18:2 note: ...but borrowed value is only valid for the block suffix following statement 0 at 16:15
<anon>:16     let x = Foo;
<anon>:17     FooRef(&x).borrow()
<anon>:18 }
