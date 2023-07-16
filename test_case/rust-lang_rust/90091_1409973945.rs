rust
if bytes.len() <= 8 {
    return Err(UnexpectedEnd);
}
// panics checked above
let (a, bytes) = bytes.split_array_ref::<4>();
let a = u32::from_be_bytes(a);

let (b, _bytes) = bytes.split_array_ref::<4>();
let b = u32::from_be_bytes(a);

Ok(Self { a, b })
