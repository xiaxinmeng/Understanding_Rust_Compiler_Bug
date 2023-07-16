rust
trait Parser<D>
where
    dyn Parser<D>: Sized,
{
    fn parse_line(&self) -> D;
}
