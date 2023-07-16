
error[E0308]: mismatched types
  --> /tmp/macro.rs:7:5
   |
7  |     noop!({
   |     ^
   |     |
   |     in this macro invocation
   |     expected integral variable, found enum `std::result::Result`
   |
   = note: expected type `{integer}`
   = note:    found type `std::result::Result<_, _>`
   = note: this error originates in a macro outside of the current crate
