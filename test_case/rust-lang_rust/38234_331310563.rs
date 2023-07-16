
error[E0308]: match arms have incompatible types
  --> src/main.rs:15:5
   |
15 | /     match do_smthg() {
16 | |         Ok(FdImplementor::File(file)) => Ok(true),
17 | |         e => e
18 | |     }
   | |_____^ expected bool, found enum `FdImplementor`
   |
   = note: expected type `std::result::Result<bool, _>`
              found type `std::result::Result<FdImplementor, _>`
note: match arm with an incompatible type
  --> src/main.rs:17:14
   |
17 |         e => e
   |              ^
