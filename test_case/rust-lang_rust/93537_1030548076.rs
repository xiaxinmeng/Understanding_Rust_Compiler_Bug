plain
    Checking askama_shared v0.12.0
   Compiling askama_derive v0.11.0
    Checking askama v0.11.0
    Checking rustdoc v0.0.0 (/checkout/src/librustdoc)
error: expected one of `!`, `(`, `+`, `::`, `<`, `>`, or `as`, found `w`
    --> src/librustdoc/html/render/mod.rs:1494:17
     |
1493 | <<<<<<< HEAD
     |             - expected one of 7 possible tokens
1494 |                 w.write_str("</section>");


error: expected one of `!`, `(`, `+`, `::`, `<`, `>`, or `as`, found `write`
    --> src/librustdoc/html/render/mod.rs:1723:5
     |
1722 | <<<<<<< HEAD
     |             - expected one of 7 possible tokens
1723 |     write!(w, "<section id=\"{}\" class=\"impl has-srclink\"{}>", id, aliases);

error: could not compile `rustdoc` due to 2 previous errors
Build completed unsuccessfully in 0:03:18
