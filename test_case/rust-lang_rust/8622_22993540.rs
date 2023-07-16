
trait Fn<Arg, Ret> {
    fn call(&self, Arg) -> Ret; // `self` is the environment
}

trait FnMut<Arg, Ret> {
    fn call(&mut self, Arg) -> Ret;
}
