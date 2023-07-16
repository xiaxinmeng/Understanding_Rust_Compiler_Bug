rust
trait IntegerMax {
    const MAX: Self;
}

impl IntegerMax for u8 {
    const MAX: Self = <u8 as IntegerMax>::MAX;
}
