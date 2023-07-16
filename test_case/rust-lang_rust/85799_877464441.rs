plain
    Checking rustdoc v0.0.0 (/checkout/src/librustdoc)
error: unused variable: `span`
   --> src/librustdoc/clean/mod.rs:137:40
    |
137 |             hir::GenericBound::Unsized(span) => {
    |                                        ^^^^ help: if this is intentional, prefix it with an underscore: `_span`
    |
    = note: `-D unused-variables` implied by `-D warnings`
error: aborting due to previous error

error: could not compile `rustdoc`

