rust
(Token::Ident(id), Err(_)) => { Ok(Line::Named(&**id)) }, // *** lifetime error is here
