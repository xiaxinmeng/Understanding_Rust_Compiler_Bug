rust
use std::sync::Arc;
use std::sync::Mutex;

struct S(i32);

impl S {
    pub fn get_int(&self) -> i32 {
        self.0
    }

    pub fn bump_int(&mut self) {
        self.0 += 1;
    }
}

fn main() {
    let s = Arc::new(Mutex::new(S(0)));

    match s.lock().unwrap().get_int() {
        i => {
            s.lock().unwrap().bump_int();
        }
    };
}
