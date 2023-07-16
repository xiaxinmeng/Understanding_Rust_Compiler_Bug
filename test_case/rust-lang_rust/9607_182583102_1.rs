
<anon>:15:5: 15:14 error: the trait `Bar` is not implemented for the type `T` [E0277]
<anon>:15     x: Foo<T>
              ^~~~~~~~~
<anon>:13:10: 13:15 note: in this expansion of #[derive_Clone] (defined in <anon>)
<anon>:15:5: 15:14 help: see the detailed explanation for E0277
<anon>:15:5: 15:14 note: required by `core::clone::Clone::clone`
error: aborting due to previous error
playpen: application terminated with error code 101
