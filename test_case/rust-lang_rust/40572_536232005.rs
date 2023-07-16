rust
mod foo {
    pub struct bar;
}

fn foo(bar: i32) {
    use foo::bar;
}
