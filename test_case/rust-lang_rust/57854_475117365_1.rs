rust
impl Foo {}

fn bar<R>(r: &R) {
    std::io::Read::read_exact(&mut r, &mut [])
    //std::io::copy(&mut r, unimplemented!())
    //std::io::BufReader::new(r)
}
