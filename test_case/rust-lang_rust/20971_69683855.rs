 rust
pub trait Parser {
    type Input;
    fn parse(&mut self, input: <Self as Parser>::Input);
}
impl Parser for () {
    type Input = ();
    fn parse(&mut self, input: ()) {

    }
}
pub fn many() -> Box<Parser<Input=()> + 'static> {
    panic!()
}

fn main() {
    many()
        .parse(());
}
