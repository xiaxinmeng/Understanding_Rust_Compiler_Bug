rust
enum ThreeVariants {
    V0,
    V1,
    V2,
}

enum TwoVariants {
    V0,
    V1,
}

enum OneVariant {
    V0,
}

enum ZeroVariants {
}


fn unwrap_three(x: Result<u32, ThreeVariants>) -> u32 {
    match x {
        Ok(y)   => y,
        Err(ThreeVariants::V0) => 0,
        Err(ThreeVariants::V1) => 1,
        Err(ThreeVariants::V2) => 2,
    }
}

fn unwrap_two(x: Result<u32, TwoVariants>) -> u32 {
    match x {
        Ok(y)   => y,
        Err(TwoVariants::V0) => 0,
        Err(TwoVariants::V1) => 1,
    }
}

fn unwrap_one(x: Result<u32, OneVariant>) -> u32 {
    match x {
        Ok(y)   => y,
        Err(OneVariant::V0) => 0,
    }
}

fn unwrap_zero(x: Result<u32, ZeroVariants>) -> u32 {
    match x {
        Ok(y)   => y,
    }
}
