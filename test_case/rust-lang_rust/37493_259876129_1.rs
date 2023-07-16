
error[E0205]: the trait `Copy` may not be implemented for type `Test`
 --> file.rs:7:6
  |
7 | impl Copy for Test {}
...
4 |     MyVariant(NoCopy)
  |     ----------------- because `Test` variant `MyVariant` does not implement `Copy`
