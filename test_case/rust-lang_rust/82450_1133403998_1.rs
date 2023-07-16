rust
// build.rs
println!("cargo:rustc-check-cfg=names(foo, bar)");
