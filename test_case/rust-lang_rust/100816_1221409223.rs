plain
 ---> 35757daa00d6
Step 9/14 : COPY host-x86_64/mingw-check/reuse-requirements.txt /tmp/
 ---> Using cache
 ---> 2458ee4ed267
Step 10/14 : RUN pip3 install --no-deps --require-hashes -r /tmp/reuse-requirements.txt
 ---> c9955f11919a
Step 11/14 : COPY host-x86_64/mingw-check/validate-toolstate.sh /scripts/
 ---> Using cache
 ---> 9691752f8b52
---
configure: rust.debug-assertions := True
configure: rust.overflow-checks := True
configure: llvm.assertions      := True
configure: dist.missing-tools   := True
configure: build.configure-args := ['--enable-sccache', '--disable-manage-submodu ...
configure: writing `config.toml` in current directory
configure: 
configure: run `python /checkout/x.py --help`
Attempting with retry: make prepare
---
---- src/mem/maybe_uninit.rs - mem::maybe_uninit::MaybeUninit<T>::from_mut (line 309) stdout ----
error[E0658]: use of unstable library feature 'maybe_uninit_from_mut'
 --> src/mem/maybe_uninit.rs:313:14
  |
7 | let uninit = MaybeUninit::from_mut(&mut v);
  |
  = help: add `#![feature(maybe_uninit_from_mut)]` to the crate attributes to enable

error: aborting due to previous error
