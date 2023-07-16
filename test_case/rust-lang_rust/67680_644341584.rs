
error: field expressions cannot have generic arguments
  --> src/lib.rs:11:36
   |
11 |                          .collect::<Result<Vec<_>, io::Error>>?;
   |                                    ^^^^^^^^^^^^^^^^^^^^^^^^^^^

error[E0615]: attempted to take value of method `collect` on type `std::iter::Map<std::fs::ReadDir, [closure@src/lib.rs:10:31: 10:62]>`
  --> src/lib.rs:11:27
   |
11 |                          .collect::<Result<Vec<_>, io::Error>>?;
   |                           ^^^^^^^ method, not a field
   |
help: use parentheses to call the method
   |
11 |                          .collect::<Result<Vec<_>, io::Error>>()?;
   |                                                               ^^
