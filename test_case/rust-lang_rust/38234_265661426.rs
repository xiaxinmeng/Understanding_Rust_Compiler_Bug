
error[E0004]: non-exhaustive patterns: `Ok(TcpStream(_))` and `TcpListener(_)` not covered
  --> <anon>:15:11
   |
15 |     match do_smthg() {
   |           ^^^^^^^^^^ patterns `Ok(TcpStream(_))` and `TcpListener(_)` not covered

error: aborting due to previous error
