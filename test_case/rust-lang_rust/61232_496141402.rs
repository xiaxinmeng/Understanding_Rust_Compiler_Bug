rust
#![feature(linkage)]
mod dep {
    extern {
        #[linkage="external"]
        pub static collision: *const i32;
    }
}

#[no_mangle]
pub static _rust_extern_with_linkage_collision: i32 = 0;

fn main() {
    unsafe {
       println!("{:p}", &dep::collision);
    }
}
