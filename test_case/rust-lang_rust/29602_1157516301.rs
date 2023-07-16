rust
extern "C" {
    #[link_name = "llvm.returnaddress"]
    fn return_address(level: i32) -> *const i8;
}

println!("return_addresss is: {:#X?}", return_address(0));
