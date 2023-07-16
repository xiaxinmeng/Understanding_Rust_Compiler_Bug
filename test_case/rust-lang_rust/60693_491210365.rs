plain
travis_time:end:05bf0e76:start=1557476120354153745,finish=1557476206910210498,duration=86556056753
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
$ export AWS_ACCESS_KEY_ID=AKIA46X5W6CZEJZ6XT55
---
[00:25:39]    Compiling rustc-demangle v0.1.10
[00:25:42]    Compiling num_cpus v1.8.0
[00:25:44]    Compiling memmap v0.6.2
[00:25:47]    Compiling rustc_llvm v0.0.0 (/checkout/src/librustc_llvm)
[00:25:51] error: unused imports: `Abi as LayoutAbi`, `Size`, `TyLayout`
[00:25:51]   --> src/librustc_codegen_llvm/abi.rs:14:50
[00:25:51]    |
[00:25:51] 14 | use rustc_target::abi::{HasDataLayout, LayoutOf, Size, TyLayout, Abi as LayoutAbi};
[00:25:51]    |
[00:25:51]    = note: `-D unused-imports` implied by `-D warnings`
[00:25:51] 
[00:25:51] error: unused import: `PointerKind`
[00:25:51] error: unused import: `PointerKind`
[00:25:51]   --> src/librustc_codegen_llvm/abi.rs:16:31
[00:25:51]    |
[00:25:51] 16 | use rustc::ty::layout::{self, PointerKind};
[00:25:51]    |                               ^^^^^^^^^^^
[00:25:51] 
[00:25:51] error[E0599]: no function or associated item named `new` found for type `rustc_target::abi::call::FnType<'_, _>` in the current scope
[00:25:51]    --> src/librustc_codegen_llvm/abi.rs:550:17
[00:25:51]     |
[00:25:51] 550 |         FnType::new(&self, sig, extra_args)
[00:25:51]     |                 ^^^ function or associated item not found in `rustc_target::abi::call::FnType<'_, _>`
[00:25:51]     = help: items from traits can only be used if the trait is in scope
[00:25:51]     = note: the following trait is implemented but not in scope, perhaps add a `use` for it:
[00:25:51]             `use crate::rustc::ty::layout::FnTypeExt;`
[00:25:51] 
[00:25:51] 
[00:25:51] error[E0599]: no function or associated item named `new_vtable` found for type `rustc_target::abi::call::FnType<'_, _>` in the current scope
[00:25:51]    --> src/librustc_codegen_llvm/abi.rs:557:17
[00:25:51]     |
[00:25:51] 557 |         FnType::new_vtable(&self, sig, extra_args)
[00:25:51]     |                 ^^^^^^^^^^ function or associated item not found in `rustc_target::abi::call::FnType<'_, _>`
[00:25:51]     = help: items from traits can only be used if the trait is in scope
[00:25:51]     = note: the following trait is implemented but not in scope, perhaps add a `use` for it:
[00:25:51]             `use crate::rustc::ty::layout::FnTypeExt;`
[00:25:51] 
[00:25:51] 
[00:25:51] error[E0599]: no function or associated item named `of_instance` found for type `rustc_target::abi::call::FnType<'_, _>` in the current scope
[00:25:51]    --> src/librustc_codegen_llvm/abi.rs:560:17
[00:25:51]     |
[00:25:51] 560 |         FnType::of_instance(&self, instance)
[00:25:51]     |                 ^^^^^^^^^^^ function or associated item not found in `rustc_target::abi::call::FnType<'_, _>`
[00:25:51]     = help: items from traits can only be used if the trait is in scope
[00:25:51]     = note: the following trait is implemented but not in scope, perhaps add a `use` for it:
[00:25:51]             `use crate::rustc::ty::layout::FnTypeExt;`
[00:25:51] 
[00:25:51] 
[00:25:52] error[E0599]: no function or associated item named `new` found for type `rustc_target::abi::call::FnType<'_, _>` in the current scope
[00:25:52]     |
[00:25:52]     |
[00:25:52] 103 |         let fty = FnType::new(self, sig, &[]);
[00:25:52]     |                           ^^^ function or associated item not found in `rustc_target::abi::call::FnType<'_, _>`
[00:25:52]     = help: items from traits can only be used if the trait is in scope
[00:25:52]     = note: the following trait is implemented but not in scope, perhaps add a `use` for it:
[00:25:52]             `use crate::rustc::ty::layout::FnTypeExt;`
[00:25:52] 
[00:25:52] 
[00:25:53] error[E0599]: no function or associated item named `new` found for type `rustc_target::abi::call::FnType<'_, _>` in the current scope
[00:25:53]    --> src/librustc_codegen_llvm/type_of.rs:242:53
[00:25:53]     |
[00:25:53] 242 |                     cx.fn_ptr_backend_type(&FnType::new(cx, sig, &[]))
[00:25:53]     |                                                     ^^^ function or associated item not found in `rustc_target::abi::call::FnType<'_, _>`
[00:25:53]     = help: items from traits can only be used if the trait is in scope
[00:25:53]     = note: the following trait is implemented but not in scope, perhaps add a `use` for it:
[00:25:53]             `use crate::rustc::ty::layout::FnTypeExt;`
[00:25:53] 
[00:25:53] 
[00:25:53] error: unused import: `FnTypeExt`
[00:25:53]   --> src/librustc_codegen_llvm/declare.rs:16:26
[00:25:53]    |
[00:25:53] 16 | use crate::abi::{FnType, FnTypeExt};
[00:25:53] 
[00:25:53] error: unused import: `FnTypeExt`
[00:25:53]  --> src/librustc_codegen_llvm/type_of.rs:1:26
[00:25:53]   |
[00:25:53]   |
[00:25:53] 1 | use crate::abi::{FnType, FnTypeExt};
[00:25:53] 
[00:25:53] error: aborting due to 9 previous errors
[00:25:53] 
[00:25:53] For more information about this error, try `rustc --explain E0599`.
---
travis_time:end:005af32d:start=1557477770108556458,finish=1557477770113085927,duration=4529469
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:0ec68c6e
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:06c5e068
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib
