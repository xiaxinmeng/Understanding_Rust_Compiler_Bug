
  --> src/main.rs:26:60
   |
26 |   async fn compile_fail_without_return() -> Result<i32, i32> {
   |  ____________________________________________________________^
27 | |     Result::<i32, i32>::Ok(20i32)?;
28 | |     //Result::<i32, i32>::Ok(20i32)
29 | | }
   | |_^ expected enum `std::result::Result`, found ()
   |
   = note: expected type `std::result::Result<i32, i32>`
              found type `()`
