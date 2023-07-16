rust

 +error[E0072]: recursive type `Foo` has infinite size
 +  --> $DIR/recursive-type-field.rs:13:1
 +   |
 +13 |   struct Foo<'a> {
 +   |  _^ starting here...
 +14 | |     bar: Bar<'a>,
 +   | |     ------------ recursive without indirection
 +15 | |     b: Rc<Bar<'a>>,
 +   | |     -------------- SHOULD WE MARK THAT IT IS RECURSIVE HERE, EVEN THOUGH IT IS NOT PART OF THE PROBLEM?
 +16 | | }
 +   | |_^ ...ending here: recursive type has infinite size
 +   |
 +   = help: insert indirection (e.g., a `Box`, `Rc`, or `&`) at some point to make `Foo` representable
