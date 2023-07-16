rust
use ::core::cell::Cell as Mut;

fn on_drop<'f, F: 'f>(f: F) -> impl 'f + Sized
// Feel free to replace the RPIT with:
//                          -> ::scopeguard::ScopeGuard<(), F>
where
    F: FnOnce(()),
{
    ::scopeguard::guard((), f)
}

fn explicit_return(b: &Mut<bool>) {
    let _local = on_drop(|()| b.set(true));
    return drop(&on_drop(|()| b.set(false)));
}

fn implicit_return(b: &Mut<bool>) {
    let _local = on_drop(|()| b.set(true));
    drop(&on_drop(|()| b.set(false)))
}

fn test(f: impl FnOnce(&Mut<bool>)) -> bool {
    let mut temporary_is_dropped_before_local = <_>::default(); /* or whatever */
    f(Mut::from_mut(&mut temporary_is_dropped_before_local));
    temporary_is_dropped_before_local
}

fn main() {
    assert_eq!(test(explicit_return), true);
    assert_eq!(test(implicit_return), false);
}
