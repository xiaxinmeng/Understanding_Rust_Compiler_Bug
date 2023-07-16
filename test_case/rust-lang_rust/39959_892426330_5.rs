
error[E0275]: overflow evaluating the requirement `&mut Vec<u8>: std::io::Write`
  |
  = help: consider adding a `#![recursion_limit="256"]` attribute to your crate (`playground`)
  = note: required because of the requirements on the impl of `std::io::Write` for `&mut &mut Vec<u8>`
  = note: 128 redundant requirements hidden
  = note: required because of the requirements on the impl of `std::io::Write` for `&mut &mut &mut &mut &mut &mut &mut &mut &mut &mut &mut &mut &mut &mut &mut &mut &mut &mut &mut &mut &mut &mut &mut &mut &mut &mut &mut &mut &mut &mut &mut &mut &mut &mut &mut &mut &mut &mut &mut &mut &mut &mut &mut &mut &mut &mut &mut &mut &mut &mut &mut &mut &mut &mut &mut &mut &mut &mut &mut &mut &mut &mut &mut &mut &mut &mut &mut &mut &mut &mut &mut &mut &mut &mut &mut &mut &mut &mut &mut &mut &mut &mut &mut &mut &mut &mut &mut &mut &mut &mut &mut &mut &mut &mut &mut &mut &mut &mut &mut &mut &mut &mut &mut &mut &mut &mut &mut &mut &mut &mut &mut &mut &mut &mut &mut &mut &mut &mut &mut &mut &mut &mut &mut &mut &mut &mut &mut &mut &mut &mut Vec<u8>`
