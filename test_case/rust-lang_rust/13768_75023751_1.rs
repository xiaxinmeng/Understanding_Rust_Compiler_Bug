
<anon>:6:27: 6:33 error: mismatched types:
 expected `[u8; 255]`,
    found `[u8; 18446744073709551615]`
(expected an array with a fixed size of 255 elements,
    found one with 18446744073709551615 elements) [E0308]
<anon>:6     let _foo: [u8; 255] = [0; P];
                                   ^~~~~~
<anon>:5:22: 5:23 error: mismatched types:
 expected `usize`,
    found `u8`
(expected usize,
    found u8) [E0308]
<anon>:5     const P: usize = O;
                              ^
