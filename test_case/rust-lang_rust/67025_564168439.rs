
error: future cannot be shared between threads safely
  --> /home/david/projects/rust/rust4/src/test/ui/async-await/issue-64130-foo.rs:18:5
   |
LL | fn require_sync<T: Sync>(val: T) {}
   |    ------------    ---- required by this bound in `require_sync`
...
LL |     require_sync(async_not_sync());
   |     ^^^^^^^^^^^^ future returned by `async_not_sync` is not `Sync`
   |
   = help: within `impl std::future::Future`, the trait `std::marker::Sync` is not implemented for `*const ()`
note: future is not `Sync` as this value is used across an await
  --> /home/david/projects/rust/rust4/src/test/ui/async-await/issue-64130-foo.rs:12:5
   |
LL |     let a = NotSync(0 as *const ());
   |         - has type `NotSync`
...
LL |     dummy().await;
   |     ^^^^^^^^^^^^^ await occurs here, with `a` maybe used later
LL | }
   | - `a` is later dropped here

error: aborting due to previous error
