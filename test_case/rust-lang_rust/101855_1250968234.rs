plain
   Compiling toml v0.5.9
error[E0308]: mismatched types
   --> doc.rs:447:13
    |
447 |             builder.src.join("src/doc/switcher.inc"),
    |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected `&OsStr`, found struct `PathBuf`
For more information about this error, try `rustc --explain E0308`.
error: could not compile `bootstrap` due to previous error
failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml
Build completed unsuccessfully in 0:01:05
