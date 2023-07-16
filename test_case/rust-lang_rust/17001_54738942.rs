 rust
mod Parser {
    enum ParseResult<T> {
        Ok(T),
    }

    struct Parser<T> {
        parse:  Box<ParseResult<T>>
    }
}

fn main(){
    let p = Parser{ parse: box Parser::Ok("world") };
}
