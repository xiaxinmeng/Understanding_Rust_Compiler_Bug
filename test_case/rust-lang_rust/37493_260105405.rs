 nocode
error[E0205]: the trait `Copy` may not be implemented for type `Foo`
  --> $DIR/issue-19950.rs:17:6
   |
14 |     MyVariant(NoCopy)
   |     ----------------- this variant doesn't implement `Copy`
...
17 | impl Copy for Foo {}
   |      ^^^^ variant `Foo::MyVariant` doesn't implement `Copy`

error[E0204]: the trait `Copy` may not be implemented for type `Bar`
  --> $DIR/issue-19950.rs:23:1
   |
23 | impl Copy for Bar {}
   | ^^^^^^^^^^^^^^^^^^^^ field `item` doesn't implement `Copy`
