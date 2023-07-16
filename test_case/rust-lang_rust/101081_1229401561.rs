rs
// from https://doc.rust-lang.org/std/mem/fn.transmute.html
// implicitly repr(Rust)
struct R<'a>(&'a i32);
unsafe fn extend_lifetime<'b>(r: R<'b>) -> R<'static> {
    std::mem::transmute::<R<'b>, R<'static>>(r)
}
// ...
