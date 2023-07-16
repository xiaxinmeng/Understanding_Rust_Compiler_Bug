 rust
pub fn main() {
    static z: &'static int = {
        let p = 3;
        &p
    };
}
