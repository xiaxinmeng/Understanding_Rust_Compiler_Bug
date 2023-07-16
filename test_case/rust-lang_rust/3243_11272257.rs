
fn main() {
    for (|| &[1, 2, 3])().each |i| {
        io::println(fmt!("%d", *i));
    }
}
