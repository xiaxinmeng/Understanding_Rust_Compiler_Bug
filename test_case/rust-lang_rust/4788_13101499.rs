
// Generic math functions:

/// Dynamically calculates the value `inf` (`1/0`).
/// Can fail on integer types.
#[inline(always)]
pub pure fn infinity<T: Num One Zero>() -> T {
    let _0: T = Zero::zero();
    let _1: T = One::one();
    _1 / _0
}

/// Dynamically calculates the value `-inf` (`-1/0`).
/// Can fail on integer types.
#[inline(always)]
pub pure fn neg_infinity<T: Num One Zero>() -> T {
    let _0: T = Zero::zero();
    let _1: T = One::one();
    - _1 / _0
}

/// Dynamically calculates the value `NaN` (`0/0`).
/// Can fail on integer types.
#[inline(always)]
pub pure fn NaN<T: Num Zero>() -> T {
    let _0: T = Zero::zero();
    _0 / _0
}

/// Returns `true` if `num` has the value `inf` (`1/0`).
/// Can fail on integer types.
#[inline(always)]
pub pure fn is_infinity<T: Num One Zero Eq>(num: &T) -> bool {
    (*num) == (infinity::<T>())
}

/// Returns `true` if `num` has the value `-inf` (`-1/0`).
/// Can fail on integer types.
#[inline(always)]
pub pure fn is_neg_infinity<T: Num One Zero Eq>(num: &T) -> bool {
    (*num) == (neg_infinity::<T>())
}

/// Returns `true` if `num` has the value `NaN` (is not equal to itself).
#[inline(always)]
pub pure fn is_NaN<T: Num Eq>(num: &T) -> bool {
    (*num) != (*num)
}

/// Returns `true` if `num` has the value `-0` (`1/num == -1/0`).
/// Can fail on integer types.
#[inline(always)]
pub pure fn is_neg_zero<T: Num One Zero Eq>(num: &T) -> bool {
    let _1: T = One::one();
    let _0: T = Zero::zero();
    *num == _0 && is_neg_infinity(&(_1 / *num))
}
