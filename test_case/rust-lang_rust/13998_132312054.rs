 Rust
fn foo(f: for<'a,'b> fn(&'a ()) -> &'a &'b ()) {
    bar(f);
}

fn bar(f: for<'a,'b> fn(&'b ()) -> &'a &'b ()) {
}

fn main() {}
