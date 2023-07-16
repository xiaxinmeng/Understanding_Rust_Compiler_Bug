rust
fn triple_add<T>(a: T, b: T, c: T) -> T

where: // <<<---- this

    T: Add<Output=T>,
