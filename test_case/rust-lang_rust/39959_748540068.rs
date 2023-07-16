
error[E0275]: overflow evaluating the requirement `std::io::Cursor<&mut Vec<u8>>: std::io::Write`
  |
  = help: consider adding a `#![recursion_limit="256"]` attribute to your crate (`foobar`)
  = note: required because of the requirements on the impl of `std::io::Write` for `&mut std::io::Cursor<&mut Vec<u8>>`
  = note: required because of the requirements on the impl of `std::io::Write` for `&mut &mut std::io::Cursor<&mut Vec<u8>>`
(and so on)

error: aborting due to previous error
