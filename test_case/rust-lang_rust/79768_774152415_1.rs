
error[E0308]: mismatched types
  --> a.rs:32:16
   |
32 |     m.bind(|x| Some(format!("{}", x)))
   |                ^^^^^^^^^^^^^^^^^^^^^^ expected associated type, found enum `Option`
   |
   = note: expected associated type `<M1 as Monad>::Plug<_>`
                         found enum `Option<String>`
   = help: consider constraining the associated type `<M1 as Monad>::Plug<_>` to `Option<String>`
   = note: for more information, visit https://doc.rust-lang.org/book/ch19-03-advanced-traits.html
