rust
pub trait MyIterator {
}

macro_rules! array_impls {
    ($($N:expr)+) => {
        $(
            impl<'a, T> MyIterator for &'a mut [T; $N] {
            }
        )+
    }
}

array_impls! { 0 1 2 }
