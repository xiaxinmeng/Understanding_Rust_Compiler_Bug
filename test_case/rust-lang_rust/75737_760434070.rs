rust
fn format_month(it: impl Iterator<Item = ()>) -> impl Iterator<Item = ()> {
    it.map(|_| ())
}
fn main() {
    let it = vec![()].into_iter();
    let it = vec![it.into_iter()].into_iter();
    let it = vec![it.into_iter()].into_iter();
    let it = it.map(|x| {
        let x = x.map(format_month);
        let _: Vec<_> = x.collect();
    });
    it.count();
}
