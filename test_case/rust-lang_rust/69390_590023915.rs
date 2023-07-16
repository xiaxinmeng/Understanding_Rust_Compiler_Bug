rust
mod foo {
    extern {
        pub fn func(x: u32);
    }
}
mod bar {
    extern {
        pub fn func();
    }
}
fn main() {
    unsafe {
        foo::func(100);
        bar::func();
    }
}
