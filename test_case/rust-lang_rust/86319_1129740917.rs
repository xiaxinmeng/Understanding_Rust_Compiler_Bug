rust
.and_then(OsStr::to_str).map_or(false, |name| name.ends_with(".tar.gz"))
