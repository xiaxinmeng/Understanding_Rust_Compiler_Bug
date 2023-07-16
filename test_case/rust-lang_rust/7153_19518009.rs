 rust
trait Printable {
        fn print(&self);
}

fn test(o: &[~Printable]) {
        for o.each |&p| {
                p.print();
        }
}

fn main() {
}
