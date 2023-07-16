
error[E0282]: type annotations needed
  --> /.../rust/src/test/ui/issues/issue-22629.rs:13:17
LL |     assert_send(Cow::Borrowed("foo"));
   |                 ^^^^^^^^^^^^^ cannot infer type for type parameter `O` declared on the enum `Cow`
