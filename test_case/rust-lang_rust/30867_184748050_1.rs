 rust
fn expects_polymorphic_function1<F>(f: F)
    where F: for<'a> IterFunction1<std::vec::Drain<'a,&'a str>> 
{
    let mut vec = vec!["abc"];
    f.apply(vec.drain(..));
}

fn main() {
    expects_polymorphic_function1(Const("hi"));
}
