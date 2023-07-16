
error[E0308]: match arms have incompatible types
  --> src/main.rs:17:14
   |
15 | /     match do_smthg() {
16 | |         Ok(FdImplementor::File(file)) => Ok(true),
   | |                                          -------- this is found to be of type `std::result::Result<_, _>`
17 | |         e => e
   | |              ^ expected bool, found enum `FdImplementor`
18 | |     }
   | |_____- `match` arms have incompatible types
   |
   = note: expected type `std::result::Result<bool, _>`
              found type `std::result::Result<FdImplementor, _>`
