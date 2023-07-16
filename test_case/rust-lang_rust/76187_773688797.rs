
// build-pass

#![feature(generic_associated_types)]
//~^ WARNING: the feature `generic_associated_types` is incomplete

fn main() {}

trait Life {
    type L<'l>: 'l + Life;
}

struct Gc<'r, T>(&'r T);

struct List<'r, T: Life>(Gc<'r, Elem<'r, T>>);

struct Elem<'r, T: Life> {
    next: List<'r, T::L<'r>>,
}
