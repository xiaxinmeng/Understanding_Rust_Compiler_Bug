rust
#![feature(const_type_name)]
#![feature(const_transmute)]
#![feature(const_panic)]

const IS_ZST: &[u8] = b" is zst";

trait ZstAssert: Sized {
    #[warn(const_err)]
    const CHECK: () = if std::mem::size_of::<Self>() == 0 {
        // would like to use the length of `IS_ZST` + the length of the
        // return value of `type_name::<Self>()` here, but that won't work because you can't use
        // generic parameters in array lengths, not even in repeat expression lengths.
        let mut y = [0_u8; 1000];
        let name: &[u8] = std::any::type_name::<Self>().as_bytes();
        let mut i = 0;
        while i < name.len() {
            y[i] = name[i];
            i += 1;
        }
        while i < IS_ZST.len() + name.len() {
            y[i] = IS_ZST[i - name.len()];
            i += 1;
        }

        let x: &str = unsafe { std::mem::transmute(&y as &[u8]) };
        panic!(x)
    };
}

impl<T: Sized> ZstAssert for T {}

fn foo<T: Sized>() {
    #[allow(path_statements)]
    {
        T::CHECK;
    }
}

fn main() {
    foo::<i32>();
    foo::<()>();
}
