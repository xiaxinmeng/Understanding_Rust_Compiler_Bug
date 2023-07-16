
impl Dummy for Variant {
    fn do_stuff(&self) {
        match *self {
            V1(a) => a.do_stuff(),
        }
    }
}
