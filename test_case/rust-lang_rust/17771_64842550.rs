 rust
trait Aaa {}

impl<'a> Aaa for &'a mut (Aaa + 'a) {}

struct Bar<'a> {
    writer: &'a mut (Aaa + 'a),
}

fn baz(_: &mut Aaa) {
}

fn foo<'a>(mut bar: Bar<'a>) {
    baz(&mut bar.writer);
}

fn main() {
}
