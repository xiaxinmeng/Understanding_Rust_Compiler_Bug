
error[E0310]: type parameter `T` may include non-`'static` lifetimes, which will not live long enough
 --> src/lib.rs:4:36
  |
4 |     let result: Box<dyn Display> = Box::new(value);
  |                                    ^^^^^^^^^^^^^^^ ...so that the type `T` will meet its required lifetime bounds
  |
help: consider excluding types with non-`'static` lifetimes by adding an explicit lifetime bound...
  |
3 | fn foo<T: Display + 'static>(value: T) -> Box<dyn Display> {
  |
