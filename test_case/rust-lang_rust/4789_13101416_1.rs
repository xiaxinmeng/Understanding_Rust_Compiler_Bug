
impl T: num::Round {
    #[inline(always)]
    pure fn round(&self, _: num::RoundMode) -> T { *self }

    #[inline(always)]
    pure fn floor(&self) -> T { *self }
    #[inline(always)]
    pure fn ceil(&self) -> T { *self }
    #[inline(always)]
    pure fn fract(&self) -> T { 0 }
}
