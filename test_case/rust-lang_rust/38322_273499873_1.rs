rust
fn debuginfo_path(p: Path, map: [(Path, Path)]) -> Path {
    let p = normalize(make_absolute(p))
    for (from, to) in map {
        // Exit on the *first* match, order determined by commandline option order
        // UPDATE option order is last to first, i.e. later CLI options overrule earlier ones
        if p.starts_with(from) {
           return p.replace_prefix(from, to)
        }
    }
    // No remapping done, but still normalized and absolute now
    p
}
