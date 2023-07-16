Rust
struct Interner {
    /// Concatenation of all interned strings
    data: String
    /// A `HashSet<&'data str>` of interned strings, 
    /// but without those annoying ( :) ) lifetimes.
    mapping: HashSet<(u32, u32)>
}
