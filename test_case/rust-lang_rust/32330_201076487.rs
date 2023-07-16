 Rust
pub struct Holder {
    pub data: Option<<for<'a> Iterator<Item=&'a u32> as Iterator>::Item>
}

fn put<'a, 'b>(h: &'a mut Holder, d: Option<&'b u32>) {
    h.data = d;
}

fn get<'a, 'b>(h: &'a Holder) -> Option<&'b u32> {
    h.data
}

fn main() {
    let mut h = Holder {
        data: None
    };
    {
        let x = 4;
        put(&mut h, Some(&x));
    }
    println!("{:?}", get(&h));
}
