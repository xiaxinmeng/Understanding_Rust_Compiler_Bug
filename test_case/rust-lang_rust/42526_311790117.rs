
error[E0277]: `Option<i32>` values cannot be be converted into `Result<i32, MyError>`
  --> foo.rs:32:13
   |
32 |     let a = method_returning_option_i32()?;
   |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ `Option`
   |
   = note: consider using the `.or_err` helper
