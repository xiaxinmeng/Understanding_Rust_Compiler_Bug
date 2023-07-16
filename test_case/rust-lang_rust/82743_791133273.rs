
pub trait HInt: Int {
    type D: DInt<H = Self> + Int;
}
pub trait DInt: Int {
    type H: HInt<D = Self> + Int;
}
