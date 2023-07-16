 plain
<anon>:6:25: 6:34 error: mismatched types:
 expected `&mut collections::string::String`,
    found `&mut collections::vec::Vec<_>`
(expected struct `collections::string::String`,
    found struct `collections::vec::Vec`) [E0308]
<anon>:6     file.read_to_string(&mut data);
                                 ^~~~~~~~~
<anon>:6:25: 6:34 help: see the detailed explanation for E0308
