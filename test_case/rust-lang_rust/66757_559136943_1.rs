rust
fn foo(e: !) -> Box<dyn std::error::Error> {
    Box::<_>::new(e)
}
