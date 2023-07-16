
error[E0595]: closure cannot assign to immutable local variable `foo`
  --> src/main.rs:10:15
   |
9  |     let foo = Foo;
   |         --- consider changing this to `mut foo`
10 |     let () = (|| {
   |               ^^ cannot borrow mutably
