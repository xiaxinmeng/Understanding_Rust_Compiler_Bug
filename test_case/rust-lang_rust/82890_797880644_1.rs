rust
use core::mem::MaybeUninit;

struct StaticString<const N: usize> {
    data: MaybeUninit<[u8; N]>,
    length: usize,
}

impl<const N: usize> StaticString<N> {
    fn try_from_str(string: &str) -> Self {
        let mut data = MaybeUninit::uninit();
        let length = string.len();

        unsafe {
            let dest = &mut data as *mut MaybeUninit<[u8; N]> as *mut u8;
            string.as_ptr().copy_to_nonoverlapping(dest, length);
        };

        Self { data, length }
    }

    fn as_slice(&self) -> &[u8] {
        unsafe {
            &*core::ptr::slice_from_raw_parts(
                &self.data as *const MaybeUninit<[u8; N]> as *const u8,
                self.length,
            )
        }
    }
}

fn main() {
    let username = StaticString::<20>::try_from_str("user");
    // works
    println!("{}  ", &unsafe { &*(username.as_slice() as *const [u8] as *const str) });  
    // doesn't work
    println!("{:?}", &unsafe { &*(username.as_slice() as *const [u8] as *const str) });  
}

