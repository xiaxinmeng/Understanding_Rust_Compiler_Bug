rust
const fn type_eq<A: 'static, B: 'static>() -> bool {
    let A = TypeId::of::<A>();
    const B: TypeId = TypeId::of::<B>(); // error[E0401]: can't use generic parameters from outer function
    match A {
        B => true,
        _ => false,
    }
}
