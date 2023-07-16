plain
   Compiling bootstrap v0.0.0 (/checkout/src/bootstrap)
error: unused variable: `target`
 --> src/bootstrap/build.rs:8:28
  |
8 | pub fn exe(name: &PathBuf, target: &str) -> PathBuf {
  |                            ^^^^^^ help: if this is intentional, prefix it with an underscore: `_target`
  |
  = note: `-D unused-variables` implied by `-D warnings`
   Compiling itoa v0.4.6
error: could not compile `bootstrap` due to previous error
warning: build failed, waiting for other jobs to finish...
error: build failed
---
   Compiling getopts v0.2.21
error: unused variable: `target`
 --> src/bootstrap/build.rs:8:28
  |
8 | pub fn exe(name: &PathBuf, target: &str) -> PathBuf {
  |                            ^^^^^^ help: if this is intentional, prefix it with an underscore: `_target`
  |
  = note: `-D unused-variables` implied by `-D warnings`
error: could not compile `bootstrap` due to previous error
warning: build failed, waiting for other jobs to finish...
error: build failed
failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
---
   Compiling filetime v0.2.14
error: unused variable: `target`
 --> src/bootstrap/build.rs:8:28
  |
8 | pub fn exe(name: &PathBuf, target: &str) -> PathBuf {
  |                            ^^^^^^ help: if this is intentional, prefix it with an underscore: `_target`
  |
  = note: `-D unused-variables` implied by `-D warnings`
error: could not compile `bootstrap` due to previous error
warning: build failed, waiting for other jobs to finish...
error: build failed
failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
---
   Compiling opener v0.5.0
error: unused variable: `target`
 --> src/bootstrap/build.rs:8:28
  |
8 | pub fn exe(name: &PathBuf, target: &str) -> PathBuf {
  |                            ^^^^^^ help: if this is intentional, prefix it with an underscore: `_target`
  |
  = note: `-D unused-variables` implied by `-D warnings`
error: could not compile `bootstrap` due to previous error
warning: build failed, waiting for other jobs to finish...
error: build failed
failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
---
   Compiling globset v0.4.5
error: unused variable: `target`
 --> src/bootstrap/build.rs:8:28
  |
8 | pub fn exe(name: &PathBuf, target: &str) -> PathBuf {
  |                            ^^^^^^ help: if this is intentional, prefix it with an underscore: `_target`
  |
  = note: `-D unused-variables` implied by `-D warnings`
error: could not compile `bootstrap` due to previous error
warning: build failed, waiting for other jobs to finish...
error: build failed
failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
