 rust
pub fn new<S: AsRef<str>>(somedata: S) -> String {
    String::from(AsRef::<str>::as_ref(&somedata))
}
