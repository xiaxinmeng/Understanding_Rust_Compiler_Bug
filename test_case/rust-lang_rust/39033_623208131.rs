
error[E0271]: type mismatch resolving `<[closure@src\main.rs:4:12: 4:36] as std::ops::FnOnce<()>>::Output == ()`                                                                                            
 --> src\main.rs:4:5
  |
1 | fn banana<F: FnOnce()>(f: F) -> ! { f(); loop {} }
  |              -------- required by this bound in `banana`
...
4 |     banana(move || -> ! { loop {} })
  |     ^^^^^^ expected `()`, found `!`
  |
  = note: expected unit type `()`
                  found type `!`
