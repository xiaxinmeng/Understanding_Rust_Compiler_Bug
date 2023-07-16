
trait Foo {
    type Bar;
    type Biff;
}

type Thing<F: Foo> = (F::Bar, F::Biff);

fn main() {
    let v: Thing<u32> = unimplemented!();
}
