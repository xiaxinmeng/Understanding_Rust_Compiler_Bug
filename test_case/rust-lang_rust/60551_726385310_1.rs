rust
trait WithAConstant {
    const SIZE: usize;
}

struct WithArray<T, U: WithAConstant> {
    data: [T; U::SIZE]
}
