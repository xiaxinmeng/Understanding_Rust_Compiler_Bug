rust
trait Bounded {
    const COUNT : usize;
}

fn make_array<B : Bounded>() -> [bool; B::COUNT] {
    [false; B::COUNT]
}
