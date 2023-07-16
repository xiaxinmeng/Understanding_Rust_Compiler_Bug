rust
struct Foo<A = (isize, char)> {
    a: A,
}

impl Foo<isize> {
    fn bar(self) -> isize {
        self.a
    }
}

impl Foo<char> {
    fn bar(self) -> char {
        self.a
    }
}

impl Foo {
    fn bar(&self) {
        let a: (isize, char) = self.a;
        let b = a.0;
        let c = a.1;

        let aa: Foo<isize> = Foo { a: b };
        let bb: isize = aa.bar();
    }
}

fn main() {
    let a = Foo { a: (123, 'a') };
    a.bar();
}
