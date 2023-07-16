
5 |     let mut file = File::create("foo.txt")?;
  |                    ^^^^^^^^^^^^^^^^^^^^^^^^ unable to convert from `std::io::Error` to `&str` (`Err(std::io::Error)` returned via `?`)
