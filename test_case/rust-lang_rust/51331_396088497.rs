rust
#[nonlocal]
pub struct S(<MyDispatch as ::repro_dispatch::Dispatch::Second<Test>>::Call);

#[local]
pub struct S(<MyDispatch as ::Dispatch::Second<Test>::Call>::Call);
