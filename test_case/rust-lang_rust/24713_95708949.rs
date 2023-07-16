 rust
#![feature(box_syntax)]
#![crate_type="lib"]

trait Clone2 {
    fn clone_convert(&self) -> Box<Clone3>;
}
trait Clone3: Clone2 {}
impl<T: Clone + 'static> Clone3 for T {}
impl<T: Clone + 'static> Clone2 for T {
    fn clone_convert(&self) -> Box<Clone3> {
        box self.clone()
    }
}
impl Clone for Box<Clone3> {
    fn clone(&self) -> Self {
        self.clone_convert()
    }
}
