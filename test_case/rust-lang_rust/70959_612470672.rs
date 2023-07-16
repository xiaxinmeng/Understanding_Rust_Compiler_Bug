
error[E0282]: type annotations needed
 --> src/main.rs:5:31
  |
5 |     let r:Rc<str> = s.chars().collect().into();
  |                               ^^^^^^^
  |                               |
  |                               cannot infer type for type parameter `B` declared on the method `collect`
  |                               help: consider specifying the type argument in the method call: `collect::<B>`
  |
  = note: type must be known at this point


