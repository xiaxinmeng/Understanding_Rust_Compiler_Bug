rust
struct OptionMutex {
    wrapping: Mutex<Option<i32>>,
}

impl OptionMutex {
    fn new(data: i32) -> Self {
        Self {
            wrapping: Mutex::new(Some(data)),
        }
    }

    fn set(&self, data: i32) {
        *self.wrapping.lock().unwrap() = None;
        // ...some slow computation...
        *self.wrapping.lock().unwrap() = Some(data);
    }

    fn get(&self) -> i32 {
        self.wrapping.lock().unwrap().expect("Oh no!")
    }
}
