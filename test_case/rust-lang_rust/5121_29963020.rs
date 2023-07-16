
trait Iterable<'a> {
    fn my_iter(&'a self);
}

fn foo<'a, C: Iterable<'a>>(c: C) {
    c.my_iter();
}

fn main() { }
