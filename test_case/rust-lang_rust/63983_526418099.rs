
error[E0532]: expected unit struct/variant or constant, found tuple variant `MyEnum::Tuple`
 --> src/lib.rs:8:9
  |
8 |         MyEnum::Tuple => "",
  |         ^^^^^^^^^^^^^ help: missing arguments: `MyEnum::Tuple(_)`

error[E0532]: expected unit struct/variant or constant, found struct variant `MyEnum::Struct`
 --> src/lib.rs:9:9
  |
9 |         MyEnum::Struct => "",
  |         ^^^^^^^^^^^^^^ help: missing struct fields: `MyEnum::Struct { s }`
