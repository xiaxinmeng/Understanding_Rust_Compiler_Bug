rust
fn curry<'a, A: 'a, B, C, F: Fn(A, B) -> C> (f: &'a F)
    -> impl Fn<(A,), Output = impl FnOnce(B) -> C + 'a> + 'a
{
    move |a| move |b| f(a,b)
}
