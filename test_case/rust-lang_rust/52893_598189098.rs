
error[E0308]: mismatched types
  --> src/main.rs:25:39
   |
3  | impl<T, F, OLIST: HList, X, Y, FINAL> AddClass<F> for Class<T>
   |            ----- this type parameter
...
25 |         let final_data = builder.push(output);
   |                                       ^^^^^^ expected type parameter `OLIST`, found struct `Class`
   |
   = note: expected type parameter `OLIST`
                      found struct `Class<OLIST>`
   = help: type parameters must be constrained to match other types
   = note: for more information, visit https://doc.rust-lang.org/book/ch10-02-traits.html#traits-as-parameters
