rust
match it {
    it: (A|B) => (), // it with type ascription (A|B)
    it @ (A|B) => (), // it binding to pattern (A|B)
}
