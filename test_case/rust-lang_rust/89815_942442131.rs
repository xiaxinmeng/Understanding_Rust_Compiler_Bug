plain
    Checking tracing-tree v0.1.9
    Checking rustdoc-json-types v0.1.0 (/checkout/src/rustdoc-json-types)
    Checking tera v1.10.0
    Checking rustdoc v0.0.0 (/checkout/src/librustdoc)
error[E0596]: cannot borrow `out` as mutable, as it is not declared as mutable
    --> src/librustdoc/html/render/mod.rs:1945:28
     |
1945 |                     write!(&mut out, "{}", line);
     |                            ^^^^^^^^ cannot borrow as mutable
note: the binding is already a mutable borrow
    --> src/librustdoc/html/render/mod.rs:1922:47
     |
     |
1922 | fn sidebar_assoc_items(cx: &Context<'_>, out: &mut Buffer, it: &clean::Item) {
     |                                               ^^^^^^^^^^^
help: try removing `&mut` here
     |
1945 -                     write!(&mut out, "{}", line);
1945 +                     write!(out, "{}", line);


error[E0596]: cannot borrow `out` as mutable, as it is not declared as mutable
    --> src/librustdoc/html/render/mod.rs:1963:28
     |
1963 |                     write!(&mut out, "{}", line);
     |                            ^^^^^^^^ cannot borrow as mutable
note: the binding is already a mutable borrow
    --> src/librustdoc/html/render/mod.rs:1922:47
     |
     |
1922 | fn sidebar_assoc_items(cx: &Context<'_>, out: &mut Buffer, it: &clean::Item) {
     |                                               ^^^^^^^^^^^
help: try removing `&mut` here
     |
1963 -                     write!(&mut out, "{}", line);
1963 +                     write!(out, "{}", line);

For more information about this error, try `rustc --explain E0596`.
error: could not compile `rustdoc` due to 2 previous errors
Build completed unsuccessfully in 0:03:01
