
error[E0221]: ambiguous associated type `Bar` in bounds of `Self`
  --> $DIR/super-trait-referencing-self-name-clash.rs:4:24
   |
LL |     type Bar;
   |     --------- ambiguous `Bar` from `Foo`
LL | }
LL | trait Qux: Foo + AsRef<Self::Bar> {
   |                        ^^^^^^^^^ ambiguous associated type `Bar`
LL |     type Bar;
   |     --------- ambiguous `Bar` from `Qux`
   |
help: use fully qualified syntax to disambiguate
   |
LL | trait Qux: Foo + AsRef<<Self as Qux>::Bar> {
   |                        ^^^^^^^^^^^^^^^^^^
help: use fully qualified syntax to disambiguate
   |
LL | trait Qux: Foo + AsRef<<Self as Foo>::Bar> {
   |                        ^^^^^^^^^^^^^^^^^^
