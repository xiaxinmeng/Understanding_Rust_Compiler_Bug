rust
use core::ops::Div;

trait CheckedDiv: Sized + Div<Self, Output = Self> {}

trait Bounded {
    fn max_value() -> Self;
}

struct Fraction;

trait TimeInt: Bounded + CheckedDiv + Div<Fraction, Output = Self> {}
trait Clock {
    type T: TimeInt;
}

trait FixedPoint {
    type T: TimeInt;
}
struct Instant<Clock: crate::Clock> {
    ticks: Clock::T,
}
impl<Clock: crate::Clock> Instant<Clock> {
    pub fn checked_add<Dur: FixedPoint>(self, duration: Dur) {
        let _ = <Clock::T as Bounded>::max_value() / 2.into();
    }
}
