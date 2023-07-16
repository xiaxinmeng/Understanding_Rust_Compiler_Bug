plain
    Checking rustdoc v0.0.0 (/checkout/src/librustdoc)
error[E0405]: cannot find trait `Write` in this scope
    --> src/librustdoc/html/render/print_item.rs:1544:29
     |
1544 | fn item_static(w: &mut impl Write, cx: &mut Context<'_>, it: &clean::Item, s: &clean::Static) {
     |
help: consider importing one of these items
     |
1    + use core::fmt::Write;
