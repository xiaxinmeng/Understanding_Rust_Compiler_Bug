 rust
...
fn check<'r, I: Iterator<uint>, T: Itble<'r, uint, I>>(cont: &'r T) -> bool
{
    // error: internal compiler error: cannot relate bound region: ReScope(83) <= ReEarlyBound(79, 0, r)
    let cont_iter = cont.iter();
    ...
}
