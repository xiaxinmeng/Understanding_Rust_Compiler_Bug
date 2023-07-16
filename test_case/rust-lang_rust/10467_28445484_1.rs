 rust
fn cat_files<'self, T: Iterator<&'self ~str>>(_: &T) {}

fn main() {
    let args = [~"foo"];
    cat_files(&args.iter().skip(1));
}
