
error[E0382]: use of moved value: `conn`
  --> src\cli\sql.rs:43:12
   |
22 |     let conn = format!("{conn}/sql");
   |         ---- move occurs because `conn` has type `std::string::String`, which does not implement the `Copy` trait
...
28 | 	 loop {
   |     ---- inside of this loop
...
43 |                     .post(conn)
   |                           ^^^^ value moved here, in previous iteration of loop
help: consider cloning the value if the performance cost is acceptable
   |
43 |                     .post(conn.clone())
   |                               ++++++++
