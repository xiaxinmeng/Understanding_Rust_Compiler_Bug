rust
fn foo(value: Option<&[String; 2]>) -> Option<Vec<String>> {
    value.map(Vec::from)
}
