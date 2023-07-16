rust
fn main() {
    struct Droppy;
    impl Drop for Droppy {
        fn drop(&mut self) {
            println!("drop")
        }
    }
    
    let f: Box<dyn Fn()> = {
        let d = Droppy;
        Box::new(move || {
            println!("start");
            let _ = d;
            println!("end");
        })
    };
    
    f();
}
