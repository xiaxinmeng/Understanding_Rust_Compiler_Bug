
mod foo {
    pub macro m($i:item, $id:ident) {
        mod bar {
            mod baz; // `foo/bar/baz.rs` or `foo/bar/baz/mod.rs`
            $i; // `baz/quux.rs` or `baz/quux/mod.rs`
            mod $id; // `baz/bar/quux.rs` or `baz/bar/quux/mod.rs` (?)
        }
    }
}

mod baz {
    ::foo::m!(mod quux;, quux);
}