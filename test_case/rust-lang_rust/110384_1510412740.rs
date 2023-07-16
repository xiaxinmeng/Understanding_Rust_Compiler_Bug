rust
let libdir_path = PathBuf::from("src")
        .join("c")
        .join("algorithm")
        // Canonicalize the path as `rustc-link-search` requires an absolute path.
        .canonicalize()
        .expect("cannot canonicalize path");
println!("cargo:rustc-link-search={}", libdir_path.to_str().unwrap());
