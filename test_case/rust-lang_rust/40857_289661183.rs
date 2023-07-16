rust
error[E0072]: recursive type `Bar` has infinite size
  --> $DIR/recursive-type-field.rs:18:1
   |
18 |   struct Bar<'a> {
   |  _^ starting here...
19 | |     y: (Foo<'a>, Foo<'a>),
   | |     --------------------- recursive without indirection
20 | |     z: Option<Bar<'a>>,
21 | |     a: &'a Foo<'a>,
22 | |     c: &'a [Bar<'a>],
23 | |     d: [Bar<'a>; 1],
   | |     --------------- recursive without indirection
24 | |     e: Foo<'a>,
   | |     ---------- recursive without indirection
25 | |     x: Bar<'a>,
   | |     ---------- recursive without indirection
26 | | }
   | |_^ ...ending here: recursive type has infinite size
   |
   = help: insert indirection (e.g., a `Box`, `Rc`, or `&`) at some point to make `Bar` representable
