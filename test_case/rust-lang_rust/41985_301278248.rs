rust
trait Get {
    fn get() -> Self;
}

trait Use {
    fn use_up(self);
}

fn parse<'a, P>()
    where P: Get, &'a P: Use, P: 'a
{
    let p = P::get();
    p.use_up();
}
