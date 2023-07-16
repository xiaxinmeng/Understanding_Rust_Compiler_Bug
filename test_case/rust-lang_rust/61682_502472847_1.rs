rust
type FixedAlias = Option<u8>;

fn work_on_fixed_alias(x: Option<u8>) -> u8 {
    match x {
        FixedAlias::Some(y) => y + 1,
        FixedAlias::None => 0,
    }
}
