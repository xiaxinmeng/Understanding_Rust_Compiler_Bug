rust
#![feature(adt_const_params)]
#![allow(incomplete_features)]

// Everything inside this module is perma-unstable.
pub mod autoref {
    // See below how this is used as a const-generic parameter.
    #[derive(Eq, PartialEq)]
    pub struct UnstableMethodSeal;

    pub trait AutoRef {
        // Not the best name but trying to avoid conflicts as much as possible.
        #[inline(always)]
        fn __rustc_unstable_auto_ref_mut_helper<
            // This is the clever bit: no stable method could have this parameter:
            const _SEAL: UnstableMethodSeal,
        >(&mut self) -> &mut Self { self }
    }
    impl<T: ?Sized> AutoRef for T {}

    // Appropriate allow_internal_unstable attributes go here:
    #[macro_export]
    macro_rules! autoref_mut {
        ($x:expr) => {{
            use $crate::autoref::AutoRef as _;
            $x.__rustc_unstable_auto_ref_mut_helper::<{$crate::autoref::UnstableMethodSeal}>()
        }}
    }
}

// Tests:
fn assert_ref_mut<T: ?Sized>(_: &mut T) {}

pub fn by_value<T>(mut x: T, f: impl FnOnce() -> T) {
    assert_ref_mut(autoref_mut!(x));
    assert_ref_mut(autoref_mut!(f()));
}
pub fn by_ref<T: ?Sized>(r: &mut T, f: impl FnOnce(&()) -> &mut T) {
    assert_ref_mut(autoref_mut!(r));
    assert_ref_mut(autoref_mut!(f(&())));
}
