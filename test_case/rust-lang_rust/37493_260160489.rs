
error[E0205]: the trait `Copy` may not be implemented for type `Foo`
 --> foo.rs:7:6
  |
4 |         MyVariant(NoCopy)
  |         ----------------- this variant doesn't implement `Copy`
...
7 | impl Copy for Foo {}
  |      ^^^^ variant `Foo::MyVariant` doesn't implement `Copy`

error[E0204]: the trait `Copy` may not be implemented for type `Bar`
  --> foo.rs:13:1
   |
9  | struct Bar {
   | - field `item` in this struct doesn't implement `Copy`
...
13 | impl Copy for Bar {}
   | ^^^^^^^^^^^^^^^^^^^^ field `item` doesn't implement `Copy`

error: aborting due to 2 previous errors
