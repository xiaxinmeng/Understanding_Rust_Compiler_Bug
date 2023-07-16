rust
macro m($i:ident) {
    fn $i(_: <type>) {} // <type> must be accessible from the use site (enforced today)
    fn f(_: <type>){} // <type> must be accessible from the use site *or* the def site
}
