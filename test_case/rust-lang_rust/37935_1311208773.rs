
error[E0308]: mismatched types
  --> src/main.rs:13:13
   |
13 | /             match x {
14 | |                 Foo::A => true,
15 | |                 Foo::B => false,
16 | |                 _ => unreachable!()
17 | |             }
   | |             ^- help: consider using a semicolon here: `;`
   | |_____________|
   |               expected `()`, found `bool`
