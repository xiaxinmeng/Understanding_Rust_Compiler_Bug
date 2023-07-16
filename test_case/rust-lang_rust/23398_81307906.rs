 rust
use std::old_io::timer;
use std::time::Duration;
use std::thread;

trait Action {
    fn one<F>(&self, f: F) where F: FnOnce() + Send {
        self.two(f);
    }

    fn two<F>(&self, f: F) where F: FnOnce() + Send;
}

pub struct Zomg(Vec<u32>);

impl Zomg {
    fn action(&self) {
        self.one(|| println!("SIZE: {}", self.0.len()));
    }
}

impl Action for Zomg {
    fn two<F>(&self, f: F) where F: FnOnce() + Send + 'static {
        thread::spawn(move || {
            timer::sleep(Duration::milliseconds(50));
            f()
        });
    }
}

pub fn main() {
    let z = Zomg(vec![1, 2, 3, 4]);
    z.action();
    drop(z);

    let mut v = Vec::new();

    for i in 0..1_000_000 {
        v.push("ZOOOOOMG".to_string());
    }

    timer::sleep(Duration::milliseconds(400));
}
