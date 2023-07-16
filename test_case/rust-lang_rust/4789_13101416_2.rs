
impl float: num::Round {
    #[inline(always)]
    pure fn round(&self, mode: num::RoundMode) -> float {
        match mode {
            num::RoundDown
                => f64::floor(*self as f64) as float,
            num::RoundUp
                => f64::ceil(*self as f64) as float,
            num::RoundToZero if is_negative(*self)
                => f64::ceil(*self as f64) as float,
            num::RoundToZero
                => f64::floor(*self as f64) as float,
            num::RoundFromZero if is_negative(*self)
                => f64::floor(*self as f64) as float,
            num::RoundFromZero
                => f64::ceil(*self as f64) as float
        }
    }

    #[inline(always)]
    pure fn floor(&self) -> float { f64::floor(*self as f64) as float}
    #[inline(always)]
    pure fn ceil(&self) -> float { f64::ceil(*self as f64) as float}
    #[inline(always)]
    pure fn fract(&self) -> float {
        if is_negative(*self) {
            (*self) - (f64::ceil(*self as f64) as float)
        } else {
            (*self) - (f64::floor(*self as f64) as float)
        }
    }
}
