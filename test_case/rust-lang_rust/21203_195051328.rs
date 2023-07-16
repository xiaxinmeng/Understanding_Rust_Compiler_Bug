 rust
#[no_mangle]
pub fn bogus<'a>(x: &'a mut ()) where &'a mut (): Clone {
    <&'a mut ()>::clone(&x);
}
