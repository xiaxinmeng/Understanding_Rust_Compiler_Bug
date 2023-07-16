rust
pub struct Ref<'a, T: Session + 'a> {
    socket:   &'a mut T,
    consumed: bool,
}

impl<'a, T: Session + 'a> Ref<'a, T> {
    pub fn into_inner(mut ref_: Self) -> &'a mut T {
        ref_.consumed = true;
        ref_.socket
    }
}

impl<'a, T: Session> Drop for Ref<'a, T> {
    fn drop(&mut self) {
        if !self.consumed {
            Session::finish(self.socket);
        }
    }
}
