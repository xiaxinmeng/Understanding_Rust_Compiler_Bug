
error[E0308]: mismatched types
 --> f54.rs:2:5
  |
1 | fn wat<T>(t: &T) -> T {
  |        -            - expected `T` because of return type
  |        |
  |        this type parameter
2 |     t.clone()
  |     ^^^^^^^^^ expected type parameter `T`, found `&T`
  |
  = note: expected type parameter `T`
                  found reference `&T`
note: `T` does not implement `Clone`, so `&T` was cloned instead
 --> f54.rs:2:5
  |
2 |     t.clone()
  |     ^
