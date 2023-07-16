rust
duplicate! {
    // The macro argument is parsed as an unstructured token stream, `_` is parsed as a plain `_` token, gensym is not created.
    // Duplication results in two plain `_` tokens.
    // Two plain `_`s are parsed after the macro is expanded, two different gensyms are created, no name conflict happens during resolution.
    use foo as _;
}

// The macro argument is parsed as an item, gensym is created when parsing `_`.
// Duplication results in two identical gensyms.
// Two gensyms are parsed after the macro is expanded, name conflict happens during resolution.
#[duplicate]
use foo as _;
