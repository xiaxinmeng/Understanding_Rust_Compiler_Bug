 rust
fn main() {
    use std::thread;
    let ref foo = [(0, 0, 0, 0)];
    vec![thread::Builder::new().spawn(|| ()).ok()];
    {
        let mut res = (0, 0, 0, 0);
        for s in foo {
            res = (s.0, s.1, s.2, s.3);
        }
        println!("{}{}", res.0, res.1);
    }
}
