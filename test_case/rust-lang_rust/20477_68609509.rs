 rust
/Users/sfackler/rust/uuid/src/lib.rs:480:1: 485:2 error: type parameter `T` must also appear as a type parameter of some type defined within this crate
/Users/sfackler/rust/uuid/src/lib.rs:480 impl<T: Encoder<E>, E> Encodable<T, E> for Uuid {
/Users/sfackler/rust/uuid/src/lib.rs:481     /// Encode a UUID as a hyphenated string
/Users/sfackler/rust/uuid/src/lib.rs:482     fn encode(&self, e: &mut T) -> Result<(), E> {
/Users/sfackler/rust/uuid/src/lib.rs:483         e.emit_str(self.to_hyphenated_string().as_slice())
/Users/sfackler/rust/uuid/src/lib.rs:484     }
/Users/sfackler/rust/uuid/src/lib.rs:485 }
