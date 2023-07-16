
<anon>:3:5: 6:6 error: mismatched types:
 expected `()`,
    found `usize`
(expected (),
    found usize) [E0308]
<anon>:3     match std::io::stdin().read_line(&mut input) {
<anon>:4         Ok(bytes_read) => bytes_read,
<anon>:5         Err(why) => 42
<anon>:6     }
