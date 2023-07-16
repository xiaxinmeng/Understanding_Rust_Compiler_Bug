bash
touch foo.rs
rustc --crate-type=rlib -Csplit-debuginfo=packed -Clinker-plugin-lto foo.rs
