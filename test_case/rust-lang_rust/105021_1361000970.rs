rust
path.as_mut_os_string().push("-suffix");

// equivalent to:
let mut os_string = std::mem::take(path).into_os_string();
os_string.push("-suffix");
*path = PathBuf::from(os_string);
