
trait Foo {
    fn call(&self, s: &str, t: &str);
}

impl<F: Fn(&str, &str)> Foo for F  {
    fn call(&self, s: &str, t: &str) {
        self(s,t);
    }
}

fn foo<F : Fn(&str, &str)>(f: F) {
    f("", "");
}

fn foo2<F: Foo>(f: F) {
    f.call("", "");
}

fn make_foo<'a, F: Fn(&str, &str) + 'a>(f: F) -> Box<dyn Foo + 'a> {
    Box::new(f)
}

fn foo3(f: Box<dyn Foo>) {
    f.call("", "");
}

fn main() {
    foo(|x,y| ()); // OK
    foo2(|x:&_,y:&_| ()); // OK
    foo2(|x,y| ()); // Error, but works with one argument
    foo3(Box::new(&|x,y| ())); // Error, even with one argument
    foo3(make_foo(|x,y| ())); // OK!
}
