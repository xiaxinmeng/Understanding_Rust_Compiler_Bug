 rust
macro_rules! define_CovariantType(
    ($($a:attr)*) => (
        #[lang="covariant_type"]
        $($a)*
        pub struct CovariantType<T>;
    )
)

#[cfg(with_alloc)] define_CovariantType!(#[deriving(Eq,Clone)])
#[cfg(not(with_alloc))] define_CovariantType!()
