
error[[E0425]](https://doc.rust-lang.org/beta/error-index.html#E0425): cannot find function `a` in this scope
 --> src/main.rs:2:5
  |
2 |     a(1, 1i32 == 2u32)
  |     ^ not found in this scope

error[[E0308]](https://doc.rust-lang.org/beta/error-index.html#E0308): mismatched types
 --> src/main.rs:2:18
  |
2 |     a(1, 1i32 == 2u32)
  |                  ^^^^ expected `i32`, found `u32`

error[[E0277]](https://doc.rust-lang.org/beta/error-index.html#E0277): can't compare `i32` with `u32`
 --> src/main.rs:2:15
  |
2 |     a(1, 1i32 == 2u32)
  |               ^^ no implementation for `i32 == u32`
  |
  = help: the trait `PartialEq<u32>` is not implemented for `i32`
  = help: the following other types implement trait `PartialEq<Rhs>`:
            <i32 as PartialEq<serde_json::Value>>
            <i32 as PartialEq<serde_yaml::Value>>
            <i32 as PartialEq>
