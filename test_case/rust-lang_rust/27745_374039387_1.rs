rust
error[E0379]: trait fns cannot be declared const
 --> src/main.rs:5:5
5 |     const fn get_type_id(&self) -> TypeId;
  |     ^^^^^ trait fns cannot be const
