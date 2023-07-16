
<anon>:10:17: 10:26 error: mismatched types:
 expected `&mut collections::string::String`,
    found `&mut collections::vec::Vec<_>`
(expected struct `collections::string::String`,
    found struct `collections::vec::Vec`) [E0308]
<anon>:10     read_object(&mut refs, &mut store);
                          ^~~~~~~~~
<anon>:10:17: 10:26 help: see the detailed explanation for E0308
