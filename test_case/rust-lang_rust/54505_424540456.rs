
match expr.node{
    Struct(Resolved(None, path, ..), _) if path.startswith("std::ops::Range") => {
    }
    _ => {}
}
