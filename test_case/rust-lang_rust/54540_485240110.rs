rust
#![type_length_limit="8388607"]

pub const MAIN: fn() = <S21 as Exp<()>>::exp as fn();

trait Exp<X> {
    fn exp();
}

impl<X> Exp<X> for () {
    fn exp() {}
}

impl<T: Exp<(X, X)>, X> Exp<X> for (T,) {
    fn exp() {
        <T as Exp<(X, X)>>::exp as fn();
    }
}

type S<T> = (T,);
type S0 = S<()>;
type S1 = S<S0>;
type S2 = S<S1>;
type S3 = S<S2>;
type S4 = S<S3>;
type S5 = S<S4>;
type S6 = S<S5>;
type S7 = S<S6>;
type S8 = S<S7>;
type S9 = S<S8>;
type S10 = S<S9>;
type S11 = S<S10>;
type S12 = S<S11>;
type S13 = S<S12>;
type S14 = S<S13>;
type S15 = S<S14>;
type S16 = S<S15>;
type S17 = S<S16>;
type S18 = S<S17>;
type S19 = S<S18>;
type S20 = S<S19>;
type S21 = S<S20>;
