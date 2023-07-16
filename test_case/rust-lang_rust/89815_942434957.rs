plain
    Checking rustdoc v0.0.0 (/checkout/src/librustdoc)
error: format argument must be a string literal
    --> src/librustdoc/html/render/mod.rs:1945:38
     |
1945 |                     write!(&mut out, line);
     |
help: you might be missing a string literal to format with
     |
     |
1945 |                     write!(&mut out, "{}", line);

error: format argument must be a string literal
    --> src/librustdoc/html/render/mod.rs:1963:38
     |
     |
1963 |                     write!(&mut out, line);
     |
help: you might be missing a string literal to format with
     |
     |
1963 |                     write!(&mut out, "{}", line);

error: could not compile `rustdoc` due to 2 previous errors
Build completed unsuccessfully in 0:02:38
