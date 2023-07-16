rust
const _10: usize = 10;

fn write_struct<T> ()
where
    can_be_evaluated!(_10 - 0):,
{
    foo::<T, 10>(); // Warning
    foo::<T, { _10 }>(); // Error (same for `{ 10 }`)
    WithConst::<10>::foo::<T>(); // Warning
    WithConst::<{ _10 }>::foo::<T>(); // Warning
    
    WithConst::<10>::write_struct::<T>(); // OK
    WithConst::<{ _10 }>::write_struct::<T>(); // OK
}
