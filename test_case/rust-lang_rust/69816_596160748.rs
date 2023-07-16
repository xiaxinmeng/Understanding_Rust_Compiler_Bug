rust
#![feature(const_generics)]

trait IterExt: Sized + Iterator {
    fn default_for_size<const N: usize>(self) -> [Self::Item; N]
    where
        [Self::Item; N]: Default,
    {
        Default::default()
    }
}

impl<This:Iterator> IterExt for This{}

fn main(){
    const N:usize=10;
    let arr:[u32;10]=(0..10).default_for_size::<N>();
}
