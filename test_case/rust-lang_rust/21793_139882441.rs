
<anon>:27:5: 27:17 error: the trait `core::marker::Sync` is not implemented for the type `*mut libc::types::common::c95::c_void` [E0277]
<anon>:27     require_sync(&x);
              ^~~~~~~~~~~~
<anon>:27:5: 27:17 help: see the detailed explanation for E0277
<anon>:27:5: 27:17 note: `*mut libc::types::common::c95::c_void` cannot be shared between threads safely
<anon>:27     require_sync(&x);
              ^~~~~~~~~~~~
<anon>:27:5: 27:17 note: required because it appears within the type `OneStruct`
<anon>:27     require_sync(&x);
              ^~~~~~~~~~~~
<anon>:27:5: 27:17 note: required because it appears within the type `TwoStruct`
<anon>:27     require_sync(&x);
              ^~~~~~~~~~~~
<anon>:27:5: 27:17 note: required by `require_sync`
<anon>:27     require_sync(&x);
              ^~~~~~~~~~~~
