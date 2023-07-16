rust
fn bar<'a, 'b>(read: Option<&'a mut (dyn Read + 'b)>) where 'b: 'a {
    ...
}
