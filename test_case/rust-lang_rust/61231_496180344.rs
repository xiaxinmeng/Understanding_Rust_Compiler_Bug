rust
#[no_mangle]
#[linkage = "external"]
static BAZ: i32 = 21;

// ...

fn main() {
    unsafe {
       assert_eq!(what(), BAZ);
    }
}
