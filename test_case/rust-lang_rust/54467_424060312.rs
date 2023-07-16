Rust
pub trait Stream {
    type Item;
    type Error;
}

pub trait ParseError<I> {
    type Output;
}

pub struct Lex<'a, I>
    where I: Stream,
          I::Error: ParseError<char>,
          <<I as Stream>::Error as ParseError<char>>::Output: 'a
{
    x: &'a <I::Error as ParseError<char>>::Output
}

pub struct Reserved<'a, I> where
    I: Stream<Item=char> + 'a,
    I::Error: ParseError<I::Item>,
// uncomment this to make it fail on stable too
//    <<I as Stream>::Error as ParseError<char>>::Output: 'a
    
{
    x: Lex<'a, I>
}

fn main() {}
