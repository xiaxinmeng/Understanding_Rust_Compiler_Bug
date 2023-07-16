plain
    Checking rustdoc v0.0.0 (/checkout/src/librustdoc)
error: unused variable: `kind`
    --> src/librustdoc/passes/collect_intra_doc_links.rs:1262:59
     |
1262 |             let (kind, id) = if let Some(UrlFragment::Def(kind, id)) = fragment {
     |                                                           ^^^^ help: if this is intentional, prefix it with an underscore: `_kind`
     |
     = note: `-D unused-variables` implied by `-D warnings`
error: could not compile `rustdoc` due to previous error
Build completed unsuccessfully in 0:03:20
