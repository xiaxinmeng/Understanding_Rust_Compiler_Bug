rust
let (a, bytes) = bytes.split_array_ref::<4>().ok_or(UnexpectedEnd)?;
let a = u32::from_be_bytes(a);

let (b, _bytes) = bytes.split_array_ref::<4>().ok_or(UnexpectedEnd)?;
let b = u32::from_be_bytes(a);

Ok(Self { a, b })
