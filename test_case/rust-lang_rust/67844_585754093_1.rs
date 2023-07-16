rust
type Curried<'a, A: 'a, B, C, F: Fn(A, B) -> C>
 = impl Fn<(A,), Output = impl FnOnce(B) -> C + 'a> + 'a;

fn curry<'a, A: 'a, B, C, F: Fn(A, B) -> C> (f: &'a F)
    -> Curried<'a, A, B, C, F>
{
    move |a| move |b| f(a,b)
}
