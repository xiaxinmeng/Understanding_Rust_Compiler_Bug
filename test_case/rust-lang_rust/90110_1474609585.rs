rust

trait Parser<D>
where
    Parser<D>: Sized,
{
    fn parse_line(&self) -> D;
}

