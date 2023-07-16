plain
    Checking rustdoc v0.0.0 (/checkout/src/librustdoc)
error[E0308]: mismatched types
    --> src/librustdoc/html/render/print_item.rs:1545:15
     |
1544 | fn item_static(w: &mut impl fmt::Write, cx: &mut Context<'_>, it: &clean::Item, s: &clean::Static) {
     |                        --------------- this type parameter
1545 |     wrap_item(w, |w| {
     |     --------- ^ expected `&mut Buffer`, found `&mut impl fmt::Write`
     |     arguments to this function are incorrect
     |
     = note: expected mutable reference `&mut html::format::Buffer`
                found mutable reference `&mut impl fmt::Write`
                found mutable reference `&mut impl fmt::Write`
note: function defined here
    --> src/librustdoc/html/render/print_item.rs:1647:4
     |
1647 | fn wrap_item<F>(w: &mut Buffer, f: F)

error[E0308]: mismatched types
    --> /checkout/library/core/src/macros/mod.rs:519:9
     |
     |
517  | macro_rules! write {
     | ------------------ in this expansion of `write!`
518  |     ($dst:expr, $($arg:tt)*) => {
     |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected `()`, found `Result<(), Error>`
     |
    ::: src/librustdoc/html/render/print_item.rs:1544:100
     |
     |
1544 | fn item_static(w: &mut impl fmt::Write, cx: &mut Context<'_>, it: &clean::Item, s: &clean::Static) {
     |                                                                                                    - help: try adding a return type: `-> Result<(), std::fmt::Error>`
...
1556 |     write!(w, "{}", document(cx, it, None, HeadingOffset::H2))
     |
     = note: expected unit type `()`
                     found enum `Result<(), std::fmt::Error>`

