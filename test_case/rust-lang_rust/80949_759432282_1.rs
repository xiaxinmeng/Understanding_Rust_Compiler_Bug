
struct Dom;
impl Dom {
    fn dom_iter(&self) -> Box<dyn Iterator<Item = &()>> {
        todo!()
    }
}

fn diff<'a, M, O, N, S>(_: O, _: N, _: S)
where
    M: 'a,
    O: IntoIterator<Item = M>,
    N: IntoIterator<Item = &'a M>,
    S: IntoIterator<Item = &'a M>,
{
    todo!()
}

fn main() {
    let n = Dom.dom_iter();
    let storage = vec![];

    diff(std::iter::empty(), n, &storage);
}
