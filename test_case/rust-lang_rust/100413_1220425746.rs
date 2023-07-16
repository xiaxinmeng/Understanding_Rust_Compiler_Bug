
query pgo_use_path(_: ()) -> ContentHashedFilePath {
    eval_always
    desc { "path to .profdata file used by LLVM" }
}

#[derive(HashStable)]
struct ContentHashedFilePath {
    path: PathBuf,
    /// A fingerprint of the file contents that `path` points to.
    hash: Fingerprint,
}
