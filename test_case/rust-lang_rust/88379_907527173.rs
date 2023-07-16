plain
    Checking rustdoc v0.0.0 (/checkout/src/librustdoc)
error[E0308]: mismatched types
    --> src/librustdoc/clean/types.rs:1966:67
     |
1966 |         self.segments.iter().map(|s| s.name.as_str()).intersperse("::").collect()
     |                                                                   ^^^^ expected struct `SymbolStr`, found `&str`

error[E0277]: a value of type `std::string::String` cannot be built from an iterator over elements of type `SymbolStr`
    --> src/librustdoc/clean/types.rs:1966:73
     |
1966 |         self.segments.iter().map(|s| s.name.as_str()).intersperse("::").collect()
     |                                                                         ^^^^^^^ value of type `std::string::String` cannot be built from `std::iter::Iterator<Item=SymbolStr>`
     |
     = help: the trait `FromIterator<SymbolStr>` is not implemented for `std::string::String`
Some errors have detailed explanations: E0277, E0308.
For more information about an error, try `rustc --explain E0277`.
error: could not compile `rustdoc` due to 2 previous errors
Build completed unsuccessfully in 0:03:05
