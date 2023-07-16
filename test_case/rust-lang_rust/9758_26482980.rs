 rust
#[unsafe_no_drop_flag]
struct Foo {
    drop_flag: u32, generation: u32
}

impl Foo {
    fn take(&mut self) -> Foo {
        println!("(take) stealing from: {:?}", *self);
        let ret = Foo { drop_flag: self.drop_flag, generation: self.generation + 1 };
        self.drop_flag = 0;
        println!("(take) returning: {:?}", ret);
        ret
    }
}

impl Drop for Foo {
    fn drop(&mut self) {
        println!("destroying: {:?}", *self)
    }
}

fn main() {
    let mut x = Foo { drop_flag: 1, generation: 0 };
    let _y = x.take().take();
}
