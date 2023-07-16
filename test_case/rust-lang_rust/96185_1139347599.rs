rust
#[allow(dead_code)]
#[repr(align(8))]
enum Aligned {
    Zero = 0,
    One = 1,
}

const X: () = {
    let aligned = Aligned::Zero;
    let _val = &(aligned as u8);
};
