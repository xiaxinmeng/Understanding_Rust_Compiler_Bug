rust
pub trait MyIterator {
}

pub struct MyStruct<T>(T);

macro_rules! array_impls {
    ($($N:expr)+) => {
        $(
            impl<'a, T> MyIterator for MyStruct<&'a mut [T; $N]> {
            }
        )+
    }
}

array_impls! { 0 1 2 }
