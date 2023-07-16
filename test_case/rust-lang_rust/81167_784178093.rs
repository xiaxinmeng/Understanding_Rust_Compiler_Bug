plain
Removing intermediate container 8ef9af298d9e
 ---> a8a90aa71d7b
Step 5/10 : RUN npm install es-check -g
 ---> Running in c4a579cdd8f0
/node-v14.4.0-linux-x64/bin/es-check -> /node-v14.4.0-linux-x64/lib/node_modules/es-check/index.js

> spawn-sync@1.0.15 postinstall /node-v14.4.0-linux-x64/lib/node_modules/es-check/node_modules/spawn-sync
> node postinstall
+ es-check@5.2.0
added 95 packages from 44 contributors in 3.39s
Removing intermediate container c4a579cdd8f0
 ---> 63388fe6ed1e
---
 ---> 9d1d3b46a05a
Successfully built 9d1d3b46a05a
Successfully tagged rust-ci:latest
Built container sha256:9d1d3b46a05a974f8e223abe00d1a0cdb3166eb568e4566dd555c94b27404a2c
Uploading finished image to https://ci-caches.rust-lang.org/docker/9f2a38e35a8211f9c2c342213b6dabbf1ce1e957c3f9f4a6874af054aa93d446d1c6f8252277cb4118d76ddf7862eec7c972b385df9acf97d8b518b20c0181e6
upload failed: - to s3://rust-lang-ci-sccache2/docker/9f2a38e35a8211f9c2c342213b6dabbf1ce1e957c3f9f4a6874af054aa93d446d1c6f8252277cb4118d76ddf7862eec7c972b385df9acf97d8b518b20c0181e6 Unable to locate credentials
[CI_JOB_NAME=mingw-check]
---
   Compiling libc v0.2.85
   Compiling std v0.0.0 (/checkout/library/std)
   Compiling compiler_builtins v0.1.39
   Compiling unwind v0.0.0 (/checkout/library/unwind)
error[E0658]: cannot borrow here, since the borrowed element may contain interior mutability
    |
    |
911 |         copy_nonoverlapping(&src as *const T, dst, 1);
    |
    = note: see issue #80384 <https://github.com/rust-lang/rust/issues/80384> for more information
    = help: add `#![feature(const_refs_to_cell)]` to the crate attributes to enable


error[E0658]: cannot borrow here, since the borrowed element may contain interior mutability
     |
     |
1007 |         copy_nonoverlapping(&src as *const T as *const u8, dst as *mut u8, mem::size_of::<T>());
     |
     = note: see issue #80384 <https://github.com/rust-lang/rust/issues/80384> for more information
     = help: add `#![feature(const_refs_to_cell)]` to the crate attributes to enable


error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0658`.
error: could not compile `core`

To learn more, run the command again with --verbose.
command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "check" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--color" "always" "--features" "panic-unwind backtrace compiler-builtins-c" "--manifest-path" "/checkout/library/test/Cargo.toml" "--message-format" "json-render-diagnostics"
failed to run: /checkout/obj/build/bootstrap/debug/bootstrap check
Build completed unsuccessfully in 0:01:20
