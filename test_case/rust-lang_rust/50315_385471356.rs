 rust
let segments: Vec<_> = segments.iter().map(|seg| seg.ident).collect();
let path = ResolvePath {
    segments: &segments,
    source: None,
};
