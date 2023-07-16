
trait Eq {
    fn eq(&self, other: &Self) -> bool;
    fn ne(&self, other: &Self) -> bool;
}

trait Ord {
    fn lt(&self, other: &Self) -> bool;
    fn le(&self, other: &Self) -> bool;
    fn ge(&self, other: &Self) -> bool;
    fn gt(&self, other: &Self) -> bool;

    // These will be handled using default implementations, but can be
    // specialised, for example using cmath functions
    fn min(&self, other: &Self) -> Self;
    fn max(&self, other: &Self) -> Self;
    fn clamp(&self, mn: &Self, mx: &Self) -> Self;
}

trait Num: Eq
           Neg<Self>
           Add<Self,Self>
           Sub<Self,Self>
           Mul<Self,Self> {
    // These will be handled using default implementations
    fn abs(&self) -> Self;
    fn signum(&self) -> Self;
}

trait Real: Ord Num {
}

trait Floating: Real
                Div<Self,Self>
                Mod<Self,Self> {
    static pure fn NaN() -> Self;
    static pure fn infinity() -> Self;
    static pure fn neg_infinity() -> Self;

    pure fn is_NaN(&self) -> bool;
    pure fn is_infinite(&self) -> bool;
    pure fn is_finite(&self) -> bool;

    static pure fn pi() -> Self;
    static pure fn two_pi() -> Self;
    static pure fn frac_pi_2() -> Self;
    static pure fn frac_pi_4() -> Self;
    static pure fn frac_1_pi() -> Self;
    static pure fn frac_2_pi() -> Self;
    static pure fn frac_2_sqrtpi() -> Self;
    static pure fn sqrt2() -> Self;
    static pure fn frac_1_sqrt2() -> Self;
    static pure fn e() -> Self;
    static pure fn log2_e() -> Self;
    static pure fn log10_e() -> Self;
    static pure fn ln_2() -> Self;
    static pure fn ln_10() -> Self;

    pure fn abs_sub(&self, b: Self) -> Self;
    pure fn mul_add(&self, b: Self, c: Self) -> Self;
    pure fn modf(&self) -> (Self, Self);
    pure fn nextafter(&self, y: Self) -> Self;

    pure fn recip(&self) -> Self;

    pure fn ceil(&self) -> Self;
    pure fn floor(&self) -> Self;
    pure fn round(&self) -> Self;
    pure fn trunc(&self) -> Self;

    pure fn pow(&self, e: Self) -> Self;
    pure fn sqrt(&self) -> Self;
    pure fn invsqrt(&self) -> Self;
    pure fn cbrt(&self) -> Self;
    pure fn exp(&self) -> Self;
    pure fn expm1(&self) -> Self;
    pure fn exp2(&self) -> Self;
    pure fn frexp<I:Int>(&self) -> (Self, I);
    pure fn ldexp<I:Int>(&self, n: I) -> Self;
    pure fn ldexp_radix<I:Int>(&self, i: I) -> Self;
    pure fn ln(&self) -> Self;
    pure fn log_radix(&self) -> Self;
    pure fn ln1p(&self) -> Self;
    pure fn log2(&self) -> Self;
    pure fn log10(&self) -> Self;
    pure fn logarithm(&self, b: Self) -> Self;
    pure fn ilog_radix<I:Int>(&self) -> I;
    pure fn erf(&self) -> Self;
    pure fn erfc(&self) -> Self;
    pure fn lgamma<I:Int>(&self) -> (Self, I);
    pure fn tgamma(&self) -> Self;

    pure fn sin(&self) -> Self;
    pure fn cos(&self) -> Self;
    pure fn tan(&self) -> Self;
    pure fn sinh(&self) -> Self;
    pure fn cosh(&self) -> Self;
    pure fn tanh(&self) -> Self;
    pure fn asin(&self) -> Self;
    pure fn acos(&self) -> Self;
    pure fn atan(&self) -> Self;
    pure fn atan2(&self, b: Self) -> Self;
    pure fn hypot(&self, b: Self) -> Self;

    pure fn j0(&self) -> Self;
    pure fn j1(&self) -> Self;
    pure fn jn<I:Int>(&self, n: I) -> Self;
    pure fn y0(&self) -> Self;
    pure fn y1(&self) -> Self;
    pure fn yn<I:Int>(&self, n: I) -> Self;
}

trait Integer: Real
               SigNum
               Div<Self,Self>
               Mod<Self,Self> {
    fn div_mod(&self, other: &Self) -> (Self,Self);

    fn even(&self) -> bool;
    fn odd(&self) -> bool;
}

trait Bounded {
    static pure fn min_bound() -> Self;
    static pure fn max_bound() -> Self;
}

trait Int: Integer
           Bounded
           Not<Self>
           BitOr<Self,Self>
           BitXor<Self,Self>
           Shl<Self,Self>
           Shr<Self,Self> {
}
