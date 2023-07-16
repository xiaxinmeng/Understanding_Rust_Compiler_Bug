 rust
resource r(i: int) {}

class c {
    let dont_copy_me: r;
    let mut counter: int;

    new() { }

    fn next() -> int {
        self.counter += 1;
        ret self.counter;
    }

    fn breakit() {
        let f1 = {|| self.next()};
        let f2 = {|| self.next()};
        io::println(#fmt["%?",f1()]);
        io::println(#fmt["%?",f2()]);
    }
}

fn main() {
    let c1 = c();
    c1.next();
    c1.next();
    c1.next();
    c1.breakit();
}
