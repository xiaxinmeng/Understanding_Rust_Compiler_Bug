rust
let v: Vec<&[u8]> = vec![b"1", b"2", b"3"];
v.join(b", ") // Error: expected u8, found array of 2 elements
v.join(&b',') // works, but is missing the space char
