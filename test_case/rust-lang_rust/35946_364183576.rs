
error[E0277]: the trait bound `usize: std::convert::From<std::io::Error>` is not satisfied
 --> src/main.rs:2:5
  |
2 |     std::fs::File::open("foo")?;
  |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^ the trait `std::convert::From<std::io::Error>` is not implemented for `usize`
  |
  = help: the following implementations were found:
            <usize as std::convert::From<u8>>
  = note: required by `std::convert::From::from`
