rust
fn write_wrapped<W>(mut w: W, s: &str, recurse: bool) -> std::fmt::Result
where
    W: std::fmt::Write,
{
    if recurse {
        write_wrapped(&mut w, s, false)
    } else {
        w.write_str(s)
    }
}

fn main() {
    let mut s = String::new();
    write_wrapped(&mut s, "test", true).unwrap();
    println!("{:?}", s);
}
