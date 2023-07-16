
error[E0597]: `s` does not live long enough
  --> src/main.rs:14:22
   |
14 |     let v = Foo { s: &mut s };
   |                      ^^^^^^ borrowed value does not live long enough
15 |     let v = v; // Works if I comment this line
   |             - copying this value requires that `s` is borrowed for `'static`
16 | }
   | - `s` dropped here while still borrowed
