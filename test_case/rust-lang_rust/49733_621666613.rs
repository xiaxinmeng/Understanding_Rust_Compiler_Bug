rust
const SIZE: usize = 4096*4096;
#[derive(Copy, Clone)]
#[repr(C, align(4096))]
pub struct Pages([u8; SIZE]);

impl Default for Pages {
    fn default() -> Self {
        Pages([0u8; SIZE])
    }
}
