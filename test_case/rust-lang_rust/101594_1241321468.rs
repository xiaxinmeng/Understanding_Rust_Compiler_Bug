plain
   Compiling rustc-rayon-core v0.4.1
    Checking either v1.6.0
   Compiling log v0.4.14
   Compiling cc v1.0.73
   Compiling rustc-hash v1.1.0 (https://github.com/lqd/rustc-hash/?branch=experiment-b#b5c68e4f)
   Compiling self_cell v0.10.2
    Checking pin-project-lite v0.2.8
   Compiling unicode-width v0.1.8
    Checking bitflags v1.3.2
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
   Compiling winapi-i686-pc-windows-gnu v0.4.0
   Compiling winapi v0.3.9
   Compiling log v0.4.14
   Compiling cc v1.0.73
   Compiling rustc-hash v1.1.0 (https://github.com/lqd/rustc-hash/?branch=experiment-b#b5c68e4f)
   Compiling libc v0.2.131
    Checking scopeguard v1.1.0
    Checking bitflags v1.3.2
   Compiling self_cell v0.10.2
   Compiling self_cell v0.10.2
   Compiling unicode-width v0.1.8
    Checking pin-project-lite v0.2.8
    Checking thin-vec v0.2.8
error[E0599]: no method named `add_to_hash` found for mutable reference `&mut FxHasher` in the current scope
   --> /cargo/git/checkouts/rustc-hash-8682b5b1a8fffbb3/b5c68e4/src/lib.rs:129:14
    |
129 |         self.add_to_hash(i as usize);
    |              ^^^^^^^^^^^ method not found in `&mut FxHasher`

error[E0599]: no method named `add_to_hash` found for mutable reference `&mut FxHasher` in the current scope
   --> /cargo/git/checkouts/rustc-hash-8682b5b1a8fffbb3/b5c68e4/src/lib.rs:130:14
    |
130 |         self.add_to_hash((i >> 32) as usize);
    |              ^^^^^^^^^^^ method not found in `&mut FxHasher`
For more information about this error, try `rustc --explain E0599`.
error: could not compile `rustc-hash` due to 2 previous errors
warning: build failed, waiting for other jobs to finish...
Build completed unsuccessfully in 0:00:32
