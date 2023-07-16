 rust
        // stability attributes are promises made across crates; do not
        // check anything for crate-local usage.
        if ast_util::is_local(id) { return }
