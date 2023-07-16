 rust
let s = req.method().as_ref();
// interestingly, using the literal `"GET"` here had a different pointer address.
// i assume since the examples are compiled as a separate crate, they each had
// their own static copy of the string
if s.len() == 3 && s.as_ptr()  == ::hyper::Get.as_ref().as_ptr() {
    Next::write()
} else {
    panic!("only can handle GET")
}
