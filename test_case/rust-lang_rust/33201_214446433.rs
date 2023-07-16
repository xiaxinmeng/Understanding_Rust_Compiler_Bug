
<anon>:25:17: 25:22 error: the trait `core::marker::Sized` is not implemented for the type `std::error::Error + Sized + 'static` [E0277]
<anon>:25 #[derive(Debug, Clone)]
                          ^~~~~
<anon>:25:17: 25:22 note: in this expansion of #[derive_Clone] (defined in <anon>)
<anon>:25:17: 25:22 help: see the detailed explanation for E0277
<anon>:25:17: 25:22 note: `std::error::Error + Sized + 'static` does not have a constant size known at compile-time
<anon>:25:17: 25:22 note: required because it appears within the type `NewError`
<anon>:25:17: 25:22 note: required by `core::clone::Clone`
error: aborting due to previous error
