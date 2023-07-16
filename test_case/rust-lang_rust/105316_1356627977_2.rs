rust
let chunks = <&[[_; N]]>::try_from(slice).unwrap();
// or
let chunks = slice.try_into::<&[[_; N]]>().unwrap();
// vs
let (chunks, []) = slice.as_chunks::<N>() else { unreachable!() };
