rust
mod dont_be_lazy {
    mod middle {
        mod inner; // naively searching by trying to incrementally add the scopes and search will find…
    }
}

mod inner; // …this module file
