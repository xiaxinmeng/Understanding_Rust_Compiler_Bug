Rust
fn foo(f: for<'a,'b> fn(&'a (), &'b ()) -> &'a &'b ()) {
    bar(f);
}

fn bar(f: for<'a,'b> fn(&'b (), &'a ()) -> &'a &'b ()) {
}

fn main() {}
