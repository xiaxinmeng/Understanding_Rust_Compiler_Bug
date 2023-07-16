plain
    Checking rustdoc v0.0.0 (/checkout/src/librustdoc)
error: unused variable: `default`
   --> src/librustdoc/html/render/mod.rs:960:43
    |
960 |         clean::AssocConstItem(ref ty, ref default) => {
    |                                           ^^^^^^^ help: if this is intentional, prefix it with an underscore: `_default`
    |
    = note: `-D unused-variables` implied by `-D warnings`
error: unused variable: `default`
    --> src/librustdoc/html/render/mod.rs:1463:47
     |
     |
1463 |             clean::AssocConstItem(ref ty, ref default) => {
     |                                               ^^^^^^^ help: if this is intentional, prefix it with an underscore: `_default`
error: could not compile `rustdoc` due to 2 previous errors
Build completed unsuccessfully in 0:03:02
