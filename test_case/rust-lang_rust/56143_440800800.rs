
[00:48:29] ---- [ui] ui/lint/unreachable_pub-pub_crate.rs stdout ----
[00:48:29] diff of stderr:
[00:48:29] 
[00:48:29] 14    = help: or consider exporting it for use by other crates
[00:48:29] 15 
[00:48:29] 16 warning: unreachable `pub` item
[00:48:29] +   --> $DIR/unreachable_pub-pub_crate.rs:26:5
[00:48:29] +    |
[00:48:29] + LL |     pub use std::env::{Args}; // braced-use has different item spans than unbraced
[00:48:29] +    |     ---^^^^^^^^^^^^^^^
[00:48:29] +    |     |
[00:48:29] +    |     help: consider restricting its visibility: `pub(crate)`
[00:48:29] +    |
[00:48:29] +    = help: or consider exporting it for use by other crates
[00:48:29] + 
[00:48:29] + warning: unreachable `pub` item
[00:48:29] 18    |
[00:48:29] 18    |
[00:48:29] 19 LL |     pub use std::env::{Args}; // braced-use has different item spans than unbraced
