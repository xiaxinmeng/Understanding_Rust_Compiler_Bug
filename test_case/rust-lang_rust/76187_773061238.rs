
// build-pass

#![feature(generic_associated_types)]
//~^ WARNING: the feature `generic_associated_types` is incomplete

fn main() {}

trait Life {
    type L<'l>: 'l + Life;
}

struct Gc<'r, T>(&'r T);
impl<'r, T: Life> Life for Gc<'r, T> {
    type L<'l> = Gc<'l, T::L<'l>>;
}

impl<'r, T: Life> Life for List<'r, T> {
    type L<'l> = List<'l, T::L<'l>>;
}

impl<'r, T: Life> Life for Elem<'r, T> {
    type L<'l> = Elem<'l, T::L<'l>>;
}

struct List<'r, T: Life>(Gc<'r, Elem<'r, T>>);

struct Elem<'r, T: Life> {
    next: List<'r, T::L<'r>>,
}
