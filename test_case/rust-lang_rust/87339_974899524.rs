rust
    if let Ok(repo) = git2::Repository::discover(path) {
        if repo.workdir().map_or(false, |workdir| workdir == path) {
