
30744.rs:13:28: 13:51 error: mismatched types:
 expected `&std::path::Path`,
    found `&core::convert::AsRef<std::path::Path>`
(expected struct `std::path::Path`,
    found trait core::convert::AsRef) [E0308]
30744.rs:13     let path_ref : &Path = root.font_path.as_ref();
                                       ^~~~~~~~~~~~~~~~~~~~~~~
30744.rs:13:28: 13:51 help: run `rustc --explain E0308` to see a detailed explanation
error: aborting due to previous error
