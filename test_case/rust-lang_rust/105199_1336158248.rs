rust
trait Stream {
    type Error;
}
trait ParseError {
    type Output;
}
impl ParseError for () {
    type Output = ();
}
impl Stream for () {
    type Error = ();
}
struct Lex<I>
where
    I: Stream,
{
    x: <I::Error as ParseError>::Output,
}
struct Reserved<I> {
    y: Lex<I>,
}
fn main() {}
