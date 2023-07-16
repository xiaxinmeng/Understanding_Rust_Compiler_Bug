nocode
error[E0205]: the trait `Copy` may not be implemented for this type
 --> <anon>:7:6
  |
7 | impl Copy for Foo {}
  |      ^^^^ variant `MyVariant` does not implement `Copy`

error[E0204]: the trait `Copy` may not be implemented for this type
  --> <anon>:13:1
   |
13 | impl Copy for Bar {}
   | ^^^^^^^^^^^^^^^^^^^^ field `item` does not implement `Copy`
