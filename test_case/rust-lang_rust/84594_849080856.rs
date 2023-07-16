rust
trait TryFromU8 {
    fn try_from(x: u8) -> Result<Self, !>;
}

impl TryFromU8 for u32 {
    fn try_from(x: u8) -> Result<Self, !> {
        x as u32
    }
}

let x = u32::try_from(3);
