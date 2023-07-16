 rust
/// This procedure performs the expansion of the
/// macro_rules! macro. It parses the RHS and adds
/// an extension to the current context.
pub fn add_new_extension<'cx>(
    // ...

    // The pattern that macro_rules matches.
    // The grammar for macro_rules! is:
    // $( $lhs:mtcs => $rhs:tt );+
    // ...quasiquoting this would be nice.
    let argument_gram = vec!(
        ms(MatchSeq(vec!(
            ms(MatchNonterminal(lhs_nm, special_idents::matchers, 0u)),
            ms(MatchTok(FAT_ARROW)),
            ms(MatchNonterminal(rhs_nm, special_idents::tt, 1u))), Some(SEMI), false, 0u, 2u)),
        //to phase into semicolon-termination instead of
        //semicolon-separation
        ms(MatchSeq(vec!(ms(MatchTok(SEMI))), None, true, 2u, 2u)));
    // ...
