
error[E0277]: the trait bound `{integer}: std::io::Read` is not satisfied
 --> file.rs:4:53
  |
4 | fn send_email(req: &mut Request) -> Result<Response<impl std::io::Read>, String> {
  |                                                     ^^^^^^^^^^^^^^^^^^ the trait `std::io::Read` is not implemented for `{integer}`
5 |   Ok(Response(42))
  |   ---------------- this returned value is of type `std::result::Result<Response<{integer}>, std::string::String>`
  |
  = help: the following implementations were found:
            <&'a std::os::unix::net::UnixStream as std::io::Read>
            <&'a std::sys::unix::fd::FileDesc as std::io::Read>
            <&[u8] as std::io::Read>
            <&mut R as std::io::Read>
          and 19 others
  = note: the return type of a function must have a statically known size
