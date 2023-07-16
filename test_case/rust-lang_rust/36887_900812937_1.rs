rust
let myslice = &somevar[..];
let x = get_from_bytes(myslice); // <- x gets a type of `Vec<u8>`
let y: MyFancyType = get_from_bytes(myslice); // <- y gets a custom type implementing `FromBytes`
