rust
macro_rules! arr {
    ($item:expr; $n:expr) => {
        {
            let mut array_uninit: [_; $n] =
                [::core::mem::MaybeUninit::uninit(); $n];
            for x in &mut array_uninit {
                unsafe { ::core::ptr::write(x.as_mut_ptr(), $item) };
            }
            array_uninit.map(|mu| unsafe { mu.assume_init() })
        }
    }
}

fn main() {
    dbg!(
        arr![arr![5; 3]; 2]
    );
}
