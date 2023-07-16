
enum Token {
    LeftParen(char),
    RightParen(char),
}

#[allow(dead_code)]
fn postfix_from_infix(infix: Vec<Token>) {
    for token in infix {
        match token {
            Toekn::LeftParen(_) => {},
            _ => {}
        }
    }
}

fn main() {}
