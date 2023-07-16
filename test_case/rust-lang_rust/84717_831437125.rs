rust
let hex_int = match format!("0x{:x}", int).parse::<TokenTree>().unwrap() {
    TokenTree::Literal(literal) => literal,
    _ => unreachable!(),
};
