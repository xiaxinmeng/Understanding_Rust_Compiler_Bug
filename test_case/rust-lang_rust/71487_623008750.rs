
fn is_shebang = first_line.starts_with("#!") && first_line[2..].contains(non_whitespace_character)

fn is_rust = non_whitespace_tokens.starts_with(#, !, [)
