
pub unsafe trait NoRcTransmutes {}

impl<T: NoRcTransmutes> Rc<T> {
    pub fn data_ptr_eq(&self, other: Rc<impl NoRcTransmutes>) { .. }
    pub fn data_ptr_eq_weak(&self, other: Weak<impl NoRcTransmutes>) { .. }
}
impl<T: NoRcTransmutes> Weak<T> {
    pub fn data_ptr_eq(&self, other: Rc<impl NoRcTransmutes>) { .. }
}
