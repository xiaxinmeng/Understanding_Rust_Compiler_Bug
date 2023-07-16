
    Checking miniz_oxide v0.4.0
error[E0053]: method `poll` has an incompatible type for trait
  --> /home/joshua/.local/lib/cargo/git/checkouts/futures-rs-4ca77cb4f4f05ac4/c949076/futures-core/src/future/future_obj.rs:64:5
   |
64 |     fn poll(self: Pin<&mut Self>, waker: &Waker) -> Poll<T> {
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ types differ in mutability
   |
   = note: expected fn pointer `fn(std::pin::Pin<&mut future::future_obj::LocalFutureObj<'a, T>>, &mut std::task::Context<'_>) -> std::task::Poll<_>`
              found fn pointer `fn(std::pin::Pin<&mut future::future_obj::LocalFutureObj<'a, T>>, &std::task::Waker) -> std::task::Poll<_>`

error[E0053]: method `poll` has an incompatible type for trait
   --> /home/joshua/.local/lib/cargo/git/checkouts/futures-rs-4ca77cb4f4f05ac4/c949076/futures-core/src/future/future_obj.rs:114:5
    |
114 |     fn poll(self: Pin<&mut Self>, waker: &Waker) -> Poll<T> {
    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ types differ in mutability
    |
    = note: expected fn pointer `fn(std::pin::Pin<&mut future::future_obj::FutureObj<'a, T>>, &mut std::task::Context<'_>) -> std::task::Poll<_>`
               found fn pointer `fn(std::pin::Pin<&mut future::future_obj::FutureObj<'a, T>>, &std::task::Waker) -> std::task::Poll<_>`

error[E0308]: mismatched types
  --> /home/joshua/.local/lib/cargo/git/checkouts/futures-rs-4ca77cb4f4f05ac4/c949076/futures-core/src/future/mod.rs:86:19
   |
86 |         self.poll(waker)
   |                   ^^^^^ types differ in mutability
   |
   = note: expected mutable reference `&mut std::task::Context<'_>`
                      found reference `&std::task::Waker`

error[E0308]: mismatched types
   --> /home/joshua/.local/lib/cargo/git/checkouts/futures-rs-4ca77cb4f4f05ac4/c949076/futures-core/src/future/future_obj.rs:118:44
    |
118 |         LocalFutureObj::poll(pinned_field, waker)
    |                                            ^^^^^ types differ in mutability
    |
    = note: expected mutable reference `&mut std::task::Context<'_>`
                       found reference `&std::task::Waker`

error[E0308]: mismatched types
   --> /home/joshua/.local/lib/cargo/git/checkouts/futures-rs-4ca77cb4f4f05ac4/c949076/futures-core/src/future/future_obj.rs:165:20
    |
165 |         F::poll(p, waker)
    |                    ^^^^^ types differ in mutability
    |
    = note: expected mutable reference `&mut std::task::Context<'_>`
                       found reference `&std::task::Waker`

error[E0308]: mismatched types
   --> /home/joshua/.local/lib/cargo/git/checkouts/futures-rs-4ca77cb4f4f05ac4/c949076/futures-core/src/future/future_obj.rs:182:25
    |
182 |         F::poll(future, waker)
    |                         ^^^^^ types differ in mutability
    |
    = note: expected mutable reference `&mut std::task::Context<'_>`
                       found reference `&std::task::Waker`

error[E0308]: mismatched types
   --> /home/joshua/.local/lib/cargo/git/checkouts/futures-rs-4ca77cb4f4f05ac4/c949076/futures-core/src/future/future_obj.rs:203:26
    |
203 |             F::poll(pin, waker)
    |                          ^^^^^ types differ in mutability
    |
    = note: expected mutable reference `&mut std::task::Context<'_>`
                       found reference `&std::task::Waker`

error[E0308]: mismatched types
   --> /home/joshua/.local/lib/cargo/git/checkouts/futures-rs-4ca77cb4f4f05ac4/c949076/futures-core/src/future/future_obj.rs:225:26
    |
225 |             F::poll(pin, waker)
    |                          ^^^^^ types differ in mutability
    |
    = note: expected mutable reference `&mut std::task::Context<'_>`
                       found reference `&std::task::Waker`

error: aborting due to 8 previous errors
