 rust
    /// Constructs a floating-point number from a significand and exponent, equivalent
    /// to: `sig * 2f64.pow(exp)`
    #[inline(always)]
    fn encode(sig: f64, exp: int) -> f64 { ldexp(sig, exp as c_int) }

    /// Splits the numder into its significand and exponent
    #[inline(always)]
    fn decode(&self) -> (f64, int) {
        let mut exp = 0;
        let sig = frexp(*self, &mut exp);
        (sig, exp as int)
    }

    #[inline(always)]
    fn significand(&self) -> f64 {
        let mut _exp = 0;
        frexp(*self, &mut _exp)
    }

    #[inline(always)]
    fn exponent(&self) -> int { ilog_radix(*self) as int }
