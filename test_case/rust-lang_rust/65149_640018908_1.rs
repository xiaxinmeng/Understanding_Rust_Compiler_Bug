rust
trait Bounded {
    const COUNT : usize;
}

fn make_array<B : Bounded>() -> [bool; <B as Bounded>::COUNT] {
    [false; B::COUNT]
}
