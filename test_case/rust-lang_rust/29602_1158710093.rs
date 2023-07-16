rust
#![feature(link_llvm_intrinsics)]

extern "C" {
    #[link_name = "llvm.returnaddress"]
    fn return_address(level: i32) -> *const i8;
}

fn main() {
    println!("return_address is:  {:#X?}", unsafe { return_address(0) });
    println!("address of main is: {:#X?}", main as fn());
}
