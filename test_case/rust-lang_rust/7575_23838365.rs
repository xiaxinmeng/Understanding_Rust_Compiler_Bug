
trait CtxtFn {
    fn f9(uint) -> uint;
}

fn use_ctxtfn<T: CtxtFn>(cf: T) -> uint {
    cf.f9(342)
}

fn main() {
}
