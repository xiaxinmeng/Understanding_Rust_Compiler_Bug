
 error: using `clone` on a double-reference; this will copy the reference of type `&[u8]` instead of cloning the inner type
   --> $DIR/unnecessary_clone.rs:89:22
    |
 LL |         let _ = &mut encoded.clone();
    |                      ^^^^^^^^^^^^^^^
    |
 help: try dereferencing it
    |
 LL |         let _ = &mut &(*encoded).clone();
    |                      ^^^^^^^^^^^^^^^^^^^
 help: or try being explicit if you are sure, that you want to clone a reference
    |
 LL |         let _ = &mut <&[u8]>::clone(encoded);
    |                      ^^^^^^^^^^^^^^^^^^^^^^^
 
 error: using `clone` on a double-reference; this will copy the reference of type `&[u8]` instead of cloning the inner type
   --> $DIR/unnecessary_clone.rs:90:18
    |
 LL |         let _ = &encoded.clone();
    |                  ^^^^^^^^^^^^^^^
    |
 help: try dereferencing it
    |
 LL |         let _ = &&(*encoded).clone();
    |                  ^^^^^^^^^^^^^^^^^^^
 help: or try being explicit if you are sure, that you want to clone a reference
    |
 LL |         let _ = &<&[u8]>::clone(encoded);
    |                  ^^^^^^^^^^^^^^^^^^^^^^^
