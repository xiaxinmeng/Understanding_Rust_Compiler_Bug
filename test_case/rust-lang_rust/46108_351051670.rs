rust
extern { type C; }
impl DynSized for C {} //~ ERROR E0322
