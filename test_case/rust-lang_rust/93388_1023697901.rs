rs
fn main() {
    pub const SERVER_CERT: &[u8] = &[1, 2, 3];
    fn f(_: &mut dyn std::io::BufRead) {}
    
    // warning: taking a mutable reference to a `const` item
    f(&mut SERVER_CERT);
    
    // No warning
    let mut cert = SERVER_CERT;
    f(&mut cert);
}
