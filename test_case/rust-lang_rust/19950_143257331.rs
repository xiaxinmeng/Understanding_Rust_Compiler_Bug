
<anon>:23:5: 23:18 error: the trait `core::marker::Send` is not implemented for the type `*const ()` [E0277]
<anon>:23     thread::spawn(|| {
              ^~~~~~~~~~~~~
<anon>:23:5: 23:18 help: see the detailed explanation for E0277
<anon>:23:5: 23:18 note: `*const ()` cannot be sent between threads safely
<anon>:23:5: 23:18 note: required because it appears within the type `obscure::Obscure`
<anon>:23:5: 23:18 note: required because it appears within the type `A`
<anon>:23:5: 23:18 note: required because it appears within the type `[closure@<anon>:23:19: 25:6 me_bad:A, me_ok:&i32]`
<anon>:23:5: 23:18 note: required by `std::thread::spawn`
error: aborting due to previous error
