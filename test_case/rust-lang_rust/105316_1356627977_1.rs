rust
let u64s = get_bytes_in_a_vector()
    .as_slice()
    .try_into::<&[[u8; 8]]>(bytes)
    .map_err(|_| SomeError)?
    .into_iter()
    .map(|chunk| u64::from_le_bytes(*bytes))
    .collect::<Vec<_>>();
// vs
let bytes = get_bytes_in_a_vector();
let (chunks, []) = bytes.as_chunks() else { return Err(SomeError.into()); };
let u64s = chunks
    .into_iter()
    .map(|chunk| u64::from_le_bytes(*bytes))
    .collect::<Vec<_>>()
