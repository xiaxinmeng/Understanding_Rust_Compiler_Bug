rust
trait ZeroSized: Sized {
    #[deny(const_err)]
    const I_AM_ZERO_SIZED: i32;
    fn requires_zero_size(self);
}

impl<T: Sized> ZeroSized for T {
    const I_AM_ZERO_SIZED: i32  = [0][std::mem::size_of::<Self>()];
    fn requires_zero_size(self) {
        #![deny(const_err)]
        let _unused = Self::I_AM_ZERO_SIZED;
        println!("requires_zero_size called");
    }
}

fn main() {
    ().requires_zero_size();
    42_u32.requires_zero_size();
}
