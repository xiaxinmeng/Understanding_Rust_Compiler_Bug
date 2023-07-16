
error[E0205]: enum variant `Test::MyVariant` does not implement `Copy`
 --> file.rs:7:6
  |
4 |     MyVariant(NoCopy)
  |                        ---- because `NoCopy` is not `Copy`
...
7 | impl Copy for Test {}
  |      ^^^^ required because `Copy` is implemented for `Test` here
