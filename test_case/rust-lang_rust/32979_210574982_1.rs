
rustup run nightly rustc sdar.rs --crate-type lib -C opt-level=3 
rustup run nightly rustc foo.rs -L. -C opt-level=3 -Clto
