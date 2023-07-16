rust
struct Foo<T, U: WithAConstant>
    // Only one of these options
    where AnyArray<U::Size>: Covariant
    where AnyArray<U::Size>: CovariantOn<U::SIZE>
{
    data: [T; U::SIZE]
}
