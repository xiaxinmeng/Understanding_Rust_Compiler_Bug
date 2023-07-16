
<anon>:2:23: 2:29 error: mismatched types:
 expected `&[u8; 3]`,
    found `&'static [u8]`
(expected array of 3 elements,
    found slice) [E0308]
<anon>:2     let x: &[u8; 3] = b"abc";
                               ^~~~~~
