
fn map<A, IA: Iterable<A>, B, C: Buildable<B>>(
    input: IA, conv: fn(&A) -> B) -> C
{
    do Buildable::build |push| {
        for input.each |a| {
            push(conv(a));
        }
    }
}
