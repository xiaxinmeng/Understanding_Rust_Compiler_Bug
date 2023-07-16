rust
// Magic type integrating with borrow checker's tracking initializedness
type Uninit<Ty: Inhabitted>;

impl<T: Inhabitted> Option<Uninit<T>> {
    fn is_some(&self) -> bool {
        match x {
            Some(_) /* must be _ */ => true,
            None => false,
        }
    }
}

impl<T, U: Inhabitted> (T, Uninit<U>> {
    fn first(&mut self) -> &mut T {
        &mut x.0
    }
}
