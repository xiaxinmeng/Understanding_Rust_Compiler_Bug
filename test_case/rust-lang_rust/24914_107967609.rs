
src/test/ntcc.rs:15:1: 138:2 error: expected one of `const`, `extern`, `fn`, `pub`, `type`, `unsafe`, or `}`, found `fn lt(input: &str, pos: usize) -> Result<usize, String> {
    match peg::runtime::match_literal(input, pos, "<", 1usize) {
        Ok(pos) => { Parser::spacing(input, pos) }
        x => x,
    }
}`
