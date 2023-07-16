rust
warning: cannot specify lifetime arguments explicitly if late bound lifetime parameters are present
   --> src/rules_and_declarations.rs:458:42
    |
458 |                     parse_nested_block::<'i, 't, _, _, _>(input, move |input| parser.parse_block(prelude, input))
    |                                          ^^
    | 
   ::: src/parser.rs
    |
791 | pub fn parse_nested_block<'i: 't, 't, F, T, E>(parser: &mut Parser<'i, 't>, parse: F)
    |                                                        - the late bound lifetime parameter is introduced here
    |
    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
    = note: for more information, see issue #42868 <https://github.com/rust-lang/rust/issues/42868>
