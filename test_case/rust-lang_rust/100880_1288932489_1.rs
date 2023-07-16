rust
fn foo(value: Option<&[String; 2]>) -> Option<Vec<String>> {
    value.map(<[_; 2]>::as_slice).map(Vec::from)
}
