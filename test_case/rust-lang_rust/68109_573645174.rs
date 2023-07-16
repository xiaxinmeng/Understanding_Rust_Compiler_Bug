text
error[E0277]: `std::rc::Rc<tokio::task::local::Scheduler>` cannot be sent between threads safely
   --> src/main.rs:3:5
    |
3   |     tokio::spawn(async move {
    |     ^^^^^^^^^^^^ `std::rc::Rc<tokio::task::local::Scheduler>` cannot be sent between threads safely
    | 
   ::: /home/op/.cargo/registry/src/github.com-1ecc6299db9ec823/tokio-0.2.9/src/task/spawn.rs:124:20
    |
124 |         T::Output: Send + 'static,
    |                    ---- required by this bound in `tokio::task::spawn::spawn`
    |
    = help: within `impl std::future::Future`, the trait `std::marker::Send` is not implemented for `std::rc::Rc<tokio::task::local::Scheduler>`
    = note: required because it appears within the type `for<'r> {&'r tokio::task::local::LocalSet, impl std::future::Future, std::rc::Rc<tokio::task::local::Scheduler>, tokio::task::local::LocalFuture<impl std::future::Future>, tokio::task::local::LocalFuture<impl std::future::Future>, tokio::task::local::LocalFuture<impl std::future::Future>, ()}`
    = note: required because it appears within the type `[static generator@DefId(15:7135 ~ tokio[52aa]::task[0]::local[0]::{{impl}}[0]::run_until[0]::{{closure}}[0]) 0:&tokio::task::local::LocalSet, 1:impl std::future::Future for<'r> {&'r tokio::task::local::LocalSet, impl std::future::Future, std::rc::Rc<tokio::task::local::Scheduler>, tokio::task::local::LocalFuture<impl std::future::Future>, tokio::task::local::LocalFuture<impl std::future::Future>, tokio::task::local::LocalFuture<impl std::future::Future>, ()}]`
    = note: required because it appears within the type `std::future::GenFuture<[static generator@DefId(15:7135 ~ tokio[52aa]::task[0]::local[0]::{{impl}}[0]::run_until[0]::{{closure}}[0]) 0:&tokio::task::local::LocalSet, 1:impl std::future::Future for<'r> {&'r tokio::task::local::LocalSet, impl std::future::Future, std::rc::Rc<tokio::task::local::Scheduler>, tokio::task::local::LocalFuture<impl std::future::Future>, tokio::task::local::LocalFuture<impl std::future::Future>, tokio::task::local::LocalFuture<impl std::future::Future>, ()}]>`
    = note: required because it appears within the type `impl std::future::Future`
    = note: required because it appears within the type `impl std::future::Future`

error[E0277]: `std::rc::Rc<tokio::task::local::Scheduler>` cannot be shared between threads safely
   --> src/main.rs:3:5
    |
3   |     tokio::spawn(async move {
    |     ^^^^^^^^^^^^ `std::rc::Rc<tokio::task::local::Scheduler>` cannot be shared between threads safely
    | 
   ::: /home/op/.cargo/registry/src/github.com-1ecc6299db9ec823/tokio-0.2.9/src/task/spawn.rs:124:20
    |
124 |         T::Output: Send + 'static,
    |                    ---- required by this bound in `tokio::task::spawn::spawn`
    |
    = help: within `tokio::task::local::LocalSet`, the trait `std::marker::Sync` is not implemented for `std::rc::Rc<tokio::task::local::Scheduler>`
    = note: required because it appears within the type `tokio::task::local::LocalSet`
    = note: required because of the requirements on the impl of `std::marker::Send` for `&tokio::task::local::LocalSet`
    = note: required because it appears within the type `[static generator@DefId(15:7135 ~ tokio[52aa]::task[0]::local[0]::{{impl}}[0]::run_until[0]::{{closure}}[0]) 0:&tokio::task::local::LocalSet, 1:impl std::future::Future for<'r> {&'r tokio::task::local::LocalSet, impl std::future::Future, std::rc::Rc<tokio::task::local::Scheduler>, tokio::task::local::LocalFuture<impl std::future::Future>, tokio::task::local::LocalFuture<impl std::future::Future>, tokio::task::local::LocalFuture<impl std::future::Future>, ()}]`
    = note: required because it appears within the type `std::future::GenFuture<[static generator@DefId(15:7135 ~ tokio[52aa]::task[0]::local[0]::{{impl}}[0]::run_until[0]::{{closure}}[0]) 0:&tokio::task::local::LocalSet, 1:impl std::future::Future for<'r> {&'r tokio::task::local::LocalSet, impl std::future::Future, std::rc::Rc<tokio::task::local::Scheduler>, tokio::task::local::LocalFuture<impl std::future::Future>, tokio::task::local::LocalFuture<impl std::future::Future>, tokio::task::local::LocalFuture<impl std::future::Future>, ()}]>`
    = note: required because it appears within the type `impl std::future::Future`
    = note: required because it appears within the type `impl std::future::Future`

error[E0308]: mismatched types
 --> src/main.rs:1:1
  |
1 | #[tokio::main]
  | ^^^^^^^^^^^^^^- help: try adding a semicolon: `;`
  | |
  | expected `()`, found enum `std::result::Result`
  |
  = note: expected unit type `()`
                  found enum `std::result::Result<impl std::future::Future, tokio::task::error::JoinError>`

error: aborting due to 3 previous errors

Some errors have detailed explanations: E0277, E0308.
For more information about an error, try `rustc --explain E0277`.
error: could not compile `rust-bug`.
