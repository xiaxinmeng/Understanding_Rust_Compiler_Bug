plain
    Checking rustc_ast_lowering v0.0.0 (/checkout/compiler/rustc_ast_lowering)
error[E0308]: mismatched types
   --> compiler/rustc_parse/src/lexer/mod.rs:409:32
    |
409 |                 (token::StrRaw(n_hashes), Mode::RawStr, 2 + n, 1 + n) // r##" "##
    |                                ^^^^^^^^ expected `u16`, found `u8`
    |
help: you can convert a `u8` to a `u16`
    |
409 |                 (token::StrRaw(n_hashes.into()), Mode::RawStr, 2 + n, 1 + n) // r##" "##

error[E0308]: mismatched types
   --> compiler/rustc_parse/src/lexer/mod.rs:414:36
    |
    |
414 |                 (token::ByteStrRaw(n_hashes), Mode::RawByteStr, 3 + n, 1 + n) // br##" "##
    |                                    ^^^^^^^^ expected `u16`, found `u8`
    |
help: you can convert a `u8` to a `u16`
    |
414 |                 (token::ByteStrRaw(n_hashes.into()), Mode::RawByteStr, 3 + n, 1 + n) // br##" "##

error[E0283]: type annotations needed
    --> compiler/rustc_parse/src/lexer/mod.rs:426:38
     |
     |
426  |                     (token::Integer, sym::integer(0))
     |                                      ^^^^^^^^^^^^ cannot infer type for type `{integer}`
     |
     = note: multiple `impl`s satisfying `{integer}: ToString` found in the `alloc` crate:
             - impl ToString for i8;
             - impl ToString for u8;
note: required by a bound in `integer`
     |
     |
1899 |     pub fn integer<N: TryInto<usize> + Copy + ToString>(n: N) -> Symbol {
     |                                               ^^^^^^^^ required by this bound in `integer`
help: consider specifying the type argument in the function call
     |
426  |                     (token::Integer, sym::integer::<N>(0))

Some errors have detailed explanations: E0283, E0308.
For more information about an error, try `rustc --explain E0283`.
error: could not compile `rustc_parse` due to 3 previous errors
