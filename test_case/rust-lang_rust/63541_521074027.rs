plain
2019-08-14T00:41:25.7062462Z do so (now or later) by using -b with the checkout command again. Example:
2019-08-14T00:41:25.7062516Z 
2019-08-14T00:41:25.7062766Z   git checkout -b <new-branch-name>
2019-08-14T00:41:25.7062811Z 
2019-08-14T00:41:25.7063120Z HEAD is now at 4dc51d1c3 Auto merge of #63541 - Centril:rollup-d3q7074, r=Centril
2019-08-14T00:41:25.7225872Z ##[section]Starting: Collect CPU-usage statistics in the background
2019-08-14T00:41:25.7229234Z ==============================================================================
2019-08-14T00:41:25.7229327Z Task         : Bash
2019-08-14T00:41:25.7229415Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2019-08-14T02:00:29.3214413Z 
2019-08-14T02:00:30.1747553Z error: use of deprecated item 'core::mem::uninitialized': use `mem::MaybeUninit` instead
2019-08-14T02:00:30.1748040Z    --> src/libstd/sys/cloudabi/mutex.rs:107:41
2019-08-14T02:00:30.1748622Z     |
2019-08-14T02:00:30.1749043Z 107 |             let mut event: abi::event = mem::uninitialized();
2019-08-14T02:00:30.1749811Z     |
2019-08-14T02:00:30.1750172Z     = note: `-D deprecated` implied by `-D warnings`
2019-08-14T02:00:30.1750413Z 
2019-08-14T02:00:30.1755828Z error: use of deprecated item 'core::mem::uninitialized': use `mem::MaybeUninit` instead
---
2019-08-14T02:00:30.1757510Z 
2019-08-14T02:00:30.1763290Z error: use of deprecated item 'core::mem::uninitialized': use `mem::MaybeUninit` instead
2019-08-14T02:00:30.1763813Z   --> src/libstd/sys/cloudabi/rwlock.rs:76:41
2019-08-14T02:00:30.1764055Z    |
2019-08-14T02:00:30.1764395Z 76 |             let mut event: abi::event = mem::uninitialized();
2019-08-14T02:00:30.1765012Z 
2019-08-14T02:00:30.1771600Z error: use of deprecated item 'core::mem::uninitialized': use `mem::MaybeUninit` instead
2019-08-14T02:00:30.1772346Z   --> src/libstd/sys/cloudabi/rwlock.rs:77:38
2019-08-14T02:00:30.1772811Z    |
2019-08-14T02:00:30.1772811Z    |
2019-08-14T02:00:30.1773344Z 77 |             let mut nevents: usize = mem::uninitialized();
2019-08-14T02:00:30.1773747Z    |                                      ^^^^^^^^^^^^^^^^^^
2019-08-14T02:00:30.1773817Z 
2019-08-14T02:00:30.1834257Z error: use of deprecated item 'core::mem::uninitialized': use `mem::MaybeUninit` instead
2019-08-14T02:00:30.1834635Z    --> src/libstd/sys/cloudabi/rwlock.rs:185:41
2019-08-14T02:00:30.1835204Z     |
2019-08-14T02:00:30.1835598Z 185 |             let mut event: abi::event = mem::uninitialized();
2019-08-14T02:00:30.1836289Z 
2019-08-14T02:00:30.1837012Z error: use of deprecated item 'core::mem::uninitialized': use `mem::MaybeUninit` instead
2019-08-14T02:00:30.1837423Z    --> src/libstd/sys/cloudabi/rwlock.rs:186:38
2019-08-14T02:00:30.1837689Z     |
2019-08-14T02:00:30.1837689Z     |
2019-08-14T02:00:30.1838060Z 186 |             let mut nevents: usize = mem::uninitialized();
2019-08-14T02:00:30.1838430Z     |                                      ^^^^^^^^^^^^^^^^^^
2019-08-14T02:00:30.1838516Z 
2019-08-14T02:00:30.1838852Z error: use of deprecated item 'core::mem::uninitialized': use `mem::MaybeUninit` instead
2019-08-14T02:00:30.1839375Z   --> src/libstd/sys/cloudabi/thread.rs:75:41
2019-08-14T02:00:30.1839647Z    |
2019-08-14T02:00:30.1840017Z 75 |             let mut event: abi::event = mem::uninitialized();
2019-08-14T02:00:30.1840768Z 
2019-08-14T02:00:30.1841061Z error: use of deprecated item 'core::mem::uninitialized': use `mem::MaybeUninit` instead
2019-08-14T02:00:30.1841360Z   --> src/libstd/sys/cloudabi/thread.rs:76:38
2019-08-14T02:00:30.1841584Z    |
---
2019-08-14T02:00:32.2439626Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-cloudabi" "-j" "2" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace compiler-builtins-c" "--manifest-path" "/checkout/src/libstd/Cargo.toml" "--message-format" "json-render-diagnostics"
2019-08-14T02:00:32.2439869Z expected success, got: exit code: 101
2019-08-14T02:00:32.2444471Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap dist --target x86_64-fuchsia,aarch64-fuchsia,wasm32-unknown-unknown,wasm32-wasi,sparcv9-sun-solaris,x86_64-sun-solaris,x86_64-unknown-linux-gnux32,x86_64-unknown-cloudabi,x86_64-fortanix-unknown-sgx,nvptx64-nvidia-cuda,armv7-unknown-linux-gnueabi,armv7-unknown-linux-musleabi
2019-08-14T02:00:32.2444675Z Build completed unsuccessfully in 1:12:13
2019-08-14T02:00:34.2906551Z ##[error]Bash exited with code '1'.
2019-08-14T02:00:34.2948107Z ##[section]Starting: Upload CPU usage statistics
2019-08-14T02:00:34.2956031Z ==============================================================================
2019-08-14T02:00:34.2956113Z Task         : Bash
2019-08-14T02:00:34.2956189Z Description  : Run a Bash script on macOS, Linux, or Windows
