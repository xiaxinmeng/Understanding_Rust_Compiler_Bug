rust
// Referred to via #![path = "some_other_directory/file.cxx"]
mod middle {
    mod inner; // Where is this looked up (need to check with `rustc`)?
}
