rust
mod foo {
    extern {
       pub fn test();
    }
}

pub mod bar {
    #[no_mangle]
    pub fn test() {
        println!("test");
    }
}

fn main() {
    unsafe { foo::test(); }
}
