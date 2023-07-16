
   fn as_bytes(&self) -> &[u8]
           Converts a string slice to a byte slice.
   Examples
           Basic usage:
               let bytes = "bors".as_bytes();
               assert_eq!(b"bors", bytes);
   