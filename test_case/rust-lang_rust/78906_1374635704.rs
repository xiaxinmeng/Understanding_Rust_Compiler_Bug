
error[E0308]: mismatched types
 --> src/main.rs:3:5
  |
2 | fn foo<T: Any>() -> T {
  |        -            -
  |        |            |
  |        |            expected `T` because of return type
  |        |            help: consider using an impl return type: `impl Any`
  |        this type parameter
3 |     ""
  |     ^^ expected type parameter `T`, found `&str`
  |
  = note: expected type parameter `T`
                  found reference `&'static str`
