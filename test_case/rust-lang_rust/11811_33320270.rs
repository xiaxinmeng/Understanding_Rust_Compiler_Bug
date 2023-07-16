
[... large collection of definitions ; no uses or definitions of `mod x` nor `extern mod x` ...]
fn foo() {
   use x::a::b::c; // is this legal and implies linking to external `mod x`?  Or is it illegal?
   ...
}
