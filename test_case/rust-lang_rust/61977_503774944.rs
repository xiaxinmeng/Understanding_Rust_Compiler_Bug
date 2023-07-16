rust
let temp_dir: Box<dyn AsRef<Path>> = match matches.value_of_os("cache-dir") {
    Some(c) => Path::new(c).to_path_buf().into_boxed_path(),
    None => Box::new(TempDir::new("temp")?)
};
