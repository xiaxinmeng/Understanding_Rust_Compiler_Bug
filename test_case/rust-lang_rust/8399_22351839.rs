
impl Drop for Foo {
    fn drop(&self) {
        let this: &mut Foo = unsafe { use std::cast; cast::transmute(self) };
        while this.next.is_some() {
            do this.next.take().map_consume |mut next| {
                this.next = next.next.take();
            };
        }
    }
}
