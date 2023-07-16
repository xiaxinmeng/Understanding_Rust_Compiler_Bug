rust
//Oops, I forgot a #[derive(Clone)]
struct MyStruct {
    stuff: Vec<u8>
}

impl MyStruct {
    fn clone_then_push_then_get_len(&self) -> usize {
        let mut x = self.clone();
        x.stuff.push(3);
        x.stuff.len()
    }
}

fn main() {
    let my = MyStruct {
        stuff: vec![1, 2]
    };
    
    println!("{}", my.clone_then_push_then_get_len());
    println!("{}", my.stuff.len());
}
