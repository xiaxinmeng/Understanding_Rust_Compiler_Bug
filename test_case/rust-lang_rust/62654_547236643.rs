
error: lifetime may not live long enough
  --> src/lib.rs:11:9
   |
10 |       fn test(&self) -> Box<dyn Future<Output=()> + 'static> {
   |               - let's call the lifetime of this reference `'1`
11 | /         Box::new(async {
12 | |             foo(self).await
13 | |         })
   | |__________^ returning this value requires that `'1` must outlive `'static`

error[E0515]: cannot return value referencing function parameter `self`
  --> src/lib.rs:11:9
   |
11 | /         Box::new(async {
12 | |             foo(self).await
   | |                 ---- `self` is borrowed here
13 | |         })
   | |__________^ returns a value referencing data owned by the current function
