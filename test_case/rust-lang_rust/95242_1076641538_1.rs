rust
fn bar<'b>(read: Option<&'b mut (dyn Read + 'b)>) {
    ...
}
