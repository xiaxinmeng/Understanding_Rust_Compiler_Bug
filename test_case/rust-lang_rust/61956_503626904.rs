
fn useless_transmute<T>(x: T) -> T {
   unsafe { core::mem::transmute(x) }
}
