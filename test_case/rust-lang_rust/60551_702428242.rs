
error: generic parameters must not be used inside of non trivial constant values
 --> src/lib.rs:5:22
  |
5 |     fn foo() -> [u8; Self::N];
  |                      ^^^^^^^ non-trivial anonymous constants must not depend on the parameter `Self`
  |
  = note: type parameters are currently not permitted in anonymous constants
