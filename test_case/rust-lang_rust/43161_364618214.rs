
error[E0507]: cannot move out of borrowed content
  --> src/main.rs:11:9
   |
3  |    fn foo(self) {}
   |    ------------ `foo` defined here
...
10 |    fn bar(&self) {
   |           ----- `self` is borrowed here
11 |         self.foo.foo();
   |         ^^^^     --- moving out of `self` because `foo(...)` moves `self.foo`
   |         |
   |         cannot move out of borrowed content
