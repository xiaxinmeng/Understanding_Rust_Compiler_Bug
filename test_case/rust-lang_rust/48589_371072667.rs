rust
// `union` is a normal ident, this is not an error
union U {
    ....
}

// `union` is a raw ident, is this an error?
r#union U {
    ...
}
