rust
#![feature(const_fn_trait_bounds)]
#![feature(const_trait_impl)]
#![feature(const_trait_bound_opt_out)]
#![allow(incomplete_features)]

mod num {
    pub trait Plus {
        fn plus(self, rhs: Self) -> Self;
    }

    struct Num<T>(T);
    impl<T: ?const Plus + Copy> const Plus for Num<T> {
        fn plus(self, rhs: Self) -> Self {
            Num(self.0.plus(rhs.0))
        }
    }
}
