plain
Removing intermediate container 23cf5a3b3cf6
 ---> 6e915391977b
Step 5/10 : RUN npm install es-check -g
 ---> Running in a8d391f0f5a6
/node-v14.4.0-linux-x64/bin/es-check -> /node-v14.4.0-linux-x64/lib/node_modules/es-check/index.js

> spawn-sync@1.0.15 postinstall /node-v14.4.0-linux-x64/lib/node_modules/es-check/node_modules/spawn-sync
> node postinstall
+ es-check@5.2.1
added 95 packages from 44 contributors in 4.921s
Removing intermediate container a8d391f0f5a6
 ---> 6486e120c85e
---
 ---> 2c6c72bdfd72
Successfully built 2c6c72bdfd72
Successfully tagged rust-ci:latest
Built container sha256:2c6c72bdfd7227f398a263ec8321b30ccef9d09002bb7a44c3f84a5e1adaf918
Uploading finished image to https://ci-caches.rust-lang.org/docker/25a5e644b6627a87559016c25ac47c181200f7dc2d956124d4ec955d40afc0106a6da85a0c1d0eecccbc916d0fa70aab5683bc9fdeab73e39b17ff4ba0589243
upload failed: - to s3://rust-lang-ci-sccache2/docker/25a5e644b6627a87559016c25ac47c181200f7dc2d956124d4ec955d40afc0106a6da85a0c1d0eecccbc916d0fa70aab5683bc9fdeab73e39b17ff4ba0589243 Unable to locate credentials
[CI_JOB_NAME=mingw-check]
---
configure: rust.channel         := nightly
configure: rust.debug-assertions := True
configure: llvm.assertions      := True
configure: dist.missing-tools   := True
configure: build.configure-args := ['--enable-sccache', '--disable-manage-submodu ...
configure: writing `config.toml` in current directory
configure: 
configure: run `python /checkout/x.py --help`
configure: 
---
test tests::no ... FAILED

failures:

---- tests::no stdout ----
thread 'tests::no' panicked at 'assertion failed: `(left == right)`
 right: `2`', src/tools/jsondocck/src/tests.rs:3:5
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace



failures:
    tests::no

test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

error: test failed, to rerun pass '--bin jsondocck'

command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--locked" "--color" "always" "--manifest-path" "/checkout/src/tools/jsondocck/Cargo.toml"
expected success, got: exit code: 101

