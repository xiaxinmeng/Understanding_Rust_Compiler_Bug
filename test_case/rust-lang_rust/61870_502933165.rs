rust
error[E0282]: type annotations needed
 --> src/main.rs:5:41
  |
  |     let x: *mut (dyn Trait + 'static) = std::ptr::null_mut();
  |                                         ^^^^^^^^^^^^^^^^^^ cannot infer type for `T`
