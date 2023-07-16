rust
#[derive(Debug)]
pub enum Token {
    Identifior(String),
    Indent,
}

impl Token {
    pub fn identifier(&self) -> Option<String> {
        match self {
            Self::Identifior(str) => Some(str.clone()),
            _ => None
        }
    }
    pub fn indent(&self) -> bool {
        self == &Self::Indent
    }
}

impl PartialEq for Token {
    fn eq(&self, other: &Self) -> bool {
        match other {
            Self::Identifior(n) => {
                let ident = self.identifier();
                if ident.is_none() { return false; }
                ident.unwrap() == *n
            }
            Self::Indent => {
                self.indent()
            }
        }
    }
}

fn main() {
    let is_ident = Token::Indent == Token::Indent;
}
