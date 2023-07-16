
<anon>:3:27: 3:42 error: mismatched types:
 expected `[u8; 255]`,
    found `[u8; 18446744073709551615]`
(expected an array with a fixed size of 255 elements,
    found one with 18446744073709551615 elements) [E0308]
<anon>:3     let _foo: [u8; 255] = [0; N as usize];
                                   ^~~~~~~~~~~~~~~
