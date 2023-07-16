
objdump -T target/debug/libdylib.so  | wc -l
2250

cat Cargo.toml 
[package]
name = "dylib"
version = "0.1.0"
authors = ["m4b <m4b.github.io@gmail.com>"]

[lib]
crate-type = ["cdylib"]

[dependencies]

cat src/lib.rs 
#[no_mangle]
pub extern fn test () {
    println!("yup");
}
