
<anon>:22:13: 22:19 error: the trait `core::marker::Send` is not implemented for the type `std::error::Error + 'static` [E0277]
<anon>:22     let t = scoped(move|| {
                      ^~~~~~
<anon>:22:13: 22:19 help: see the detailed explanation for E0277
<anon>:22:13: 22:19 note: `std::error::Error + 'static` cannot be sent between threads safely
<anon>:22:13: 22:19 note: required because it appears within the type `Box<std::error::Error + 'static>`
<anon>:22:13: 22:19 note: required because it appears within the type `A`
<anon>:22:13: 22:19 note: required because it appears within the type `[closure@<anon>:22:20: 24:6 tx:std::sync::mpsc::Sender<A>]`
<anon>:22:13: 22:19 note: required by `scoped`
error: aborting due to previous error
playpen: application terminated with error code 101
