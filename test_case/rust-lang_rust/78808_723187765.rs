rust
trait T {}
impl PartialEq for dyn T + '_ {
    fn eq (self: &'_ Self, _: &'_ Self) -> bool
    {
        loop {}
    }
}

fn _check (it: &'_ Box<dyn T>)
{
    *it == *it;
}
