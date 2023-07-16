rust
#[uses(S, T)]
macro m($i:ident) {
    struct S; //~ ERROR cannot define identifiers in `#[uses()]` but not `#[defines()]`
    T // resolves at the call-site
}
