
error[E0308]: match arms have incompatible types
  --> src/main.rs:15:5
   |
17 |         e => e
   |              ^ expected bool, found enum `FdImplementor`
   = note: expected type `std::result::Result<bool, _>`
              found type `std::result::Result<FdImplementor, _>`
note: this match arm covers multiple cases
  --> src/main.rs:17:14
   |
17 |         e => e
   |         ^
