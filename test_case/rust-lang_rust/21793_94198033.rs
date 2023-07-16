
#![feature(optin_builtin_traits)]

fn main() {
println!("{} {}", 1u8.foo(), <u32 as SizeOf>::foo(&1u64))
}


trait SizeOf {
    fn foo(&self) -> u8;
}

trait SizeOfOI {}
impl SizeOfOI for .. {
}

impl<T> SizeOf for T where T: SizeOfOI {
    fn foo(&self) -> u8 {0}
}

impl !SizeOfOI for u8 {}

impl SizeOf for u8{
    fn foo(&self) -> u8 {1}
}

impl !SizeOfOI for u32 {}
