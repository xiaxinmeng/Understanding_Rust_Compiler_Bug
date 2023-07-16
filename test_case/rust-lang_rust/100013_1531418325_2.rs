
error: lifetime bound not satisfied
  --> src/lib.rs:12:9
   |
12 | /         async move {
13 | |             // asks for an unspecified lifetime to outlive itself? weird diagnostics
14 | |             self.run(t).await;
15 | |         }
   | |_________^
   |
note: the lifetime defined here...
  --> src/lib.rs:14:18
   |
14 |             self.run(t).await;
   |                  ^^^
note: ...must outlive the lifetime defined here
  --> src/lib.rs:14:18
   |
14 |             self.run(t).await;
   |                  ^^^
   = note: this is a known limitation that will be removed in the future ([see issue #100013 <https://github.com/rust-lang/rust/issues/100013>](https://github.com/rust-lang/rust/issues/100013) for more information)
