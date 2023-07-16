rust
struct PathedIoError {
    path: String,
    inner: std::io::Error,
}

impl std::fmt::Debug for PathedIoError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("PathedIoError")
            .field("path", &self.path)
            .field("path", &self.inner)
            .finish()
    }
}

fn main() {
    let e = PathedIoError {
        path: "/boo".to_owned(),
        inner: std::io::Error::from_raw_os_error(1),
    };
    println!("{e:?}");
    println!("{e:#?}");
}
