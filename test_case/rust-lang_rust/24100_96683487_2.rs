 rust
pub struct Consumer<T> {
    vec: Vec<T>
}

impl<T> Consumer<T> {
    pub fn pop(&mut self) -> T {
        self.vec.pop().unwrap() as T // <--- Removing "as T" fixes the ICE
    }
}

// #[derive(Copy, Clone)]  // <-- tried to add Copy/Clone, does not fix ICE
struct TestStruct {
    a: u32
}

impl TestStruct {
    pub fn new() -> TestStruct {
        TestStruct { a: 19 }
    }
}


#[test]
fn test_struct() {
    let mut c = Consumer {
        vec: vec![TestStruct::new(), TestStruct::new()]
        //vec: vec![0,1,2,3]        // <-- works fine
    };

    let t: TestStruct = c.pop();
}
