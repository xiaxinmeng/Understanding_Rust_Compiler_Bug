rust
#![cfg_attr(target_os = "none", no_std)]
#![cfg_attr(target_os = "none", no_main)]

#[derive(Copy, Clone)]
pub struct Scalar {
    pub bytes: [u8; 32],
}
impl Scalar {
    pub fn non_adjacent_form(&self, w: usize) -> [i8; 256] {
        let mut naf = [0i8; 256];

        let mut x_u64 = [0u64; 5];

        let x_u64_ptr_first = x_u64.as_ptr() as usize;
        println!("x_u64: {:x?}", x_u64);
        println!("&x_u64: 0x{:08x}", x_u64_ptr_first);
        println!(
            "Address is aligned? {} (remainder: {})",
            x_u64_ptr_first & 7 == 0,
            x_u64_ptr_first & 7
        );
        let x_u64_ptr_second = x_u64.as_ptr() as usize;
        assert_eq!(0, (x_u64.as_ptr() as usize) & 7);
        println!(
            "Passed assertion that 0 == {} {:08x} (or {} {:08x})",
            x_u64_ptr_first & 7,
            x_u64_ptr_first,
            x_u64_ptr_second & 7,
            x_u64_ptr_second
        );

        naf
    }
}

fn main() {
    let scalar = Scalar {
        bytes: [
            189, 59, 214, 8, 77, 86, 240, 50, 111, 170, 86, 37, 124, 154, 209, 79, 102, 72, 93, 53,
            130, 157, 102, 200, 60, 240, 215, 104, 246, 58, 214, 13,
        ],
    };
    let a_naf = scalar.non_adjacent_form(5);
}
