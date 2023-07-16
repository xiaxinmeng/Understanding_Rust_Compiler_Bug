
error[E0284]: type annotations needed: cannot satisfy `<f64 as std::ops::Div<_>>::Output == _`
  --> src/main.rs:14:34
   |
14 |         let angle: f64 = 3.14f64 / steps.into() * i.into();
   |                                  ^ cannot satisfy `<f64 as std::ops::Div<_>>::Output == _`
