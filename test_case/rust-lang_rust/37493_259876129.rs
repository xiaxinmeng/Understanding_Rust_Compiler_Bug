
error[E0205]: the trait `Copy` may not be implemented for this type
 --> file.rs:7:6
  |
4 |     MyVariant(NoCopy)
  |     ----------------- `MyVariant` defined here
...
7 | impl Copy for Test {}
  |      ^^^^ variant `MyVariant` does not implement `Copy`
