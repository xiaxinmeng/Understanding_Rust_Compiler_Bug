
// build-pass

fn main() {}

trait Life {
    type L;
}

struct List<'r, T>(&'r Elem<'r, T>);

struct Elem<'r, T: Life> {
    next: List<'r, T::L>,
}
