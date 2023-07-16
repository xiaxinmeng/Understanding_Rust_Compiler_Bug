rust
#![feature(nll)]
#![allow(dead_code)]

struct S<'a> {
    value: &'a Value,
}

struct Value {
    data: u32,
}

impl<'a> S<'a> {
    fn get(&self) -> Result<&'a mut Value, String> {
        unimplemented!();
    }

    fn at(&self)  {
        let v = self.get();
        if let Ok(Value { ref mut data }) = v {
            let _res : &'a u32 = data;
        }
    }
}

fn main() {
}
