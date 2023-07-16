rust
#[derive(Debug)]
struct NotCopy(u8);
struct Foo(Vec<NotCopy>);
impl Foo {
    fn iter(&self) -> impl Iterator<Item = (NotCopy, &NotCopy)> {
        self.0.iter().map(|x| (NotCopy(x.0), x))
    }
    fn iter2(&self) -> impl Iterator<Item = NotCopy> + '_ {
        self.iter().map(|(x, _)| x)
    }
    fn work(&mut self) {
        // let x = self.iter2().find(|_| true); if let Some(x) = x {
        if let Some(x) = self.iter2().find(|_| true) {
            self.0.push(x);
        }
    }
}

fn main() {
    let mut v = Foo(vec![NotCopy(0)]);
    v.work();
    println!("{:?}", v.0);
}
