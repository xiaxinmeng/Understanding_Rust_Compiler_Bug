rust
fn help<'a, T: Ty<'a>, F>(f: &F, t: <T as Ty<'a>>::V)
where
    F: Fn(<T as Ty>::V),
{
    f(t)
}
