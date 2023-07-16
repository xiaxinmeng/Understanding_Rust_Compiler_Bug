
fn consume_declaration_value(parser: &Parser, name: ~str) -> Declaration {
    let mut name = name; // XXX see <-> below
    let mut value: ~[Primitive] = ~[];
    for parser.each_token |token| {
        match token {
            Delim('!') => {
                let mut name_ = ~"";
                let mut value_: ~[Primitive] = ~[];
                name_ <-> name;
                value_ <-> value;
                return consume_declaration_important(parser, is_nested, name_, value_)
            },
            Semicolon => break,
            _ => value.push(process(token))
        }
    }
    // Reached a Semicolon or EOF.
    Declaration{name: name, value: value, important: false})
}
