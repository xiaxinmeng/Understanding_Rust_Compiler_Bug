 rust
#![feature(unboxed_closures)]

struct Parser<I, O> {
    parse: Box<FnMut<(I,), Result<O, String>> + 'static>
}

impl<I, O:'static> Parser<I, O> {
    fn compose<K>(mut self, mut rhs: Parser<O, K>) -> Parser<I, K> {
        Parser {
            parse: box move |&mut: x: I| {
                match self.parse.call_mut((x,)) {
                    Ok(r) => rhs.parse.call_mut((r,)),
                    Err(e) => Err(e)
                }
            }
        }
    }
}

fn main() {}
