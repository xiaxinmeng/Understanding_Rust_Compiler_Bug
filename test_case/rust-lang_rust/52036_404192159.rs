plain
  InsecurePlatformWarning
  Downloading https://files.pythonhosted.org/packages/2d/99/b2c4e9d5a30f6471e410a146232b4118e697fa3ffc06d6a65efde84debd0/futures-3.2.0-py2-none-any.whl
Requirement already satisfied: six>=1.5 in /usr/lib/python2.7/dist-packages (from python-dateutil<3.0.0,>=2.1; python_version >= "2.7"->botocore==1.10.55->awscli)
Building wheels for collected packages: awscli
  Running setup.py bdist_wheel for awscli ... - \ | / - \ done
Successfully built awscli
Installing collected packages: docutils, jmespath, python-dateutil, botocore, colorama, pyasn1, rsa, futures, s3transfer, awscli
Successfully installed awscli-1.15.56 botocore-1.10.55 colorama-0.3.9 docutils-0.14 futures-3.2.0 jmespath-0.9.3 pyasn1-0.4.3 python-dateutil-2.7.3 rsa-3.4.2 s3transfer-0.1.13
/usr/local/lib/python2.7/dist-packages/pip/_vendor/requests/packages/urllib3/util/ssl_.py:122: InsecurePlatformWarning: A true SSLContext object is not available. This prevents urllib3 from configuring SSL appropriately and may cause certain SSL connections to fail. You can upgrade to a newer version of Python to solve this. For more information, see https://urllib3.readthedocs.io/en/latest/security.html#insecureplatformwarning.
---
[00:02:24]    Compiling bootstrap v0.0.0 (file:///checkout/src/bootstrap)
[00:02:27] error[E0599]: no variant named `ToolTest` found for type `Mode` in the current scope
[00:02:27]    --> bootstrap/builder.rs:765:17
[00:02:27]     |
[00:02:27] 765 |                 Mode::ToolTest => { },
[00:02:27]     |                 ^^^^^^^^^^^^^^ variant not found in `Mode`
[00:02:27]     | 
[00:02:27]    ::: bootstrap/lib.rs:319:1
[00:02:27]     |
[00:02:27] 319 | pub enum Mode {
[00:02:27]     | ------------- variant `ToolTest` not found here
[00:02:28] error: aborting due to previous error
[00:02:28] 
[00:02:28] For more information about this error, try `rustc --explain E0599`.
[00:02:28] For more information about this error, try `rustc --explain E0599`.
[00:02:28] error: Could not compile `bootstrap`.
[00:02:28] To learn more, run the command again with --verbose.
[00:02:28] failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
[00:02:28] Build completed unsuccessfully in 0:01:29
[00:02:28] Makefile:81: recipe for target 'prepare' failed
[00:02:28] Makefile:81: recipe for target 'prepare' failed
[00:02:28] make: *** [prepare] Error 1
[00:02:29] Command failed. Attempt 2/5:
[00:02:29]    Compiling bootstrap v0.0.0 (file:///checkout/src/bootstrap)
[00:02:33] error[E0599]: no variant named `ToolTest` found for type `Mode` in the current scope
[00:02:33]    --> bootstrap/builder.rs:765:17
[00:02:33]     |
[00:02:33] 765 |                 Mode::ToolTest => { },
[00:02:33]     |                 ^^^^^^^^^^^^^^ variant not found in `Mode`
[00:02:33]     | 
[00:02:33]    ::: bootstrap/lib.rs:319:1
[00:02:33]     |
[00:02:33] 319 | pub enum Mode {
[00:02:33]     | ------------- variant `ToolTest` not found here
[00:02:33] error: aborting due to previous error
[00:02:33] 
[00:02:33] For more information about this error, try `rustc --explain E0599`.
[00:02:33] For more information about this error, try `rustc --explain E0599`.
[00:02:33] error: Could not compile `bootstrap`.
[00:02:33] To learn more, run the command again with --verbose.
[00:02:33] failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
[00:02:33] Build completed unsuccessfully in 0:00:04
[00:02:33] Makefile:81: recipe for target 'prepare' failed
[00:02:33] Makefile:81: recipe for target 'prepare' failed
[00:02:33] make: *** [prepare] Error 1
[00:02:35] Command failed. Attempt 3/5:
[00:02:36]    Compiling bootstrap v0.0.0 (file:///checkout/src/bootstrap)
[00:02:40] error[E0599]: no variant named `ToolTest` found for type `Mode` in the current scope
[00:02:40]    --> bootstrap/builder.rs:765:17
[00:02:40]     |
[00:02:40] 765 |                 Mode::ToolTest => { },
[00:02:40]     |                 ^^^^^^^^^^^^^^ variant not found in `Mode`
[00:02:40]     | 
[00:02:40]    ::: bootstrap/lib.rs:319:1
[00:02:40]     |
[00:02:40] 319 | pub enum Mode {
[00:02:40]     | ------------- variant `ToolTest` not found here
[00:02:40] error: aborting due to previous error
[00:02:40] 
[00:02:40] For more information about this error, try `rustc --explain E0599`.
[00:02:40] For more information about this error, try `rustc --explain E0599`.
[00:02:40] error: Could not compile `bootstrap`.
[00:02:40] To learn more, run the command again with --verbose.
[00:02:40] failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
[00:02:40] Build completed unsuccessfully in 0:00:04
[00:02:40] Makefile:81: recipe for target 'prepare' failed
[00:02:40] Makefile:81: recipe for target 'prepare' failed
[00:02:40] make: *** [prepare] Error 1
[00:02:43] Command failed. Attempt 4/5:
[00:02:43]    Compiling bootstrap v0.0.0 (file:///checkout/src/bootstrap)
[00:02:47] error[E0599]: no variant named `ToolTest` found for type `Mode` in the current scope
[00:02:47]    --> bootstrap/builder.rs:765:17
[00:02:47]     |
[00:02:47] 765 |                 Mode::ToolTest => { },
[00:02:47]     |                 ^^^^^^^^^^^^^^ variant not found in `Mode`
[00:02:47]     | 
[00:02:47]    ::: bootstrap/lib.rs:319:1
[00:02:47]     |
[00:02:47] 319 | pub enum Mode {
[00:02:47]     | ------------- variant `ToolTest` not found here
[00:02:48] error: aborting due to previous error
[00:02:48] 
[00:02:48] For more information about this error, try `rustc --explain E0599`.
[00:02:48] For more information about this error, try `rustc --explain E0599`.
[00:02:48] error: Could not compile `bootstrap`.
[00:02:48] To learn more, run the command again with --verbose.
[00:02:48] failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
[00:02:48] Build completed unsuccessfully in 0:00:04
[00:02:48] Makefile:81: recipe for target 'prepare' failed
[00:02:48] Makefile:81: recipe for target 'prepare' failed
[00:02:48] make: *** [prepare] Error 1
[00:02:52] Command failed. Attempt 5/5:
[00:02:52]    Compiling bootstrap v0.0.0 (file:///checkout/src/bootstrap)
[00:02:56] error[E0599]: no variant named `ToolTest` found for type `Mode` in the current scope
[00:02:56]    --> bootstrap/builder.rs:765:17
[00:02:56]     |
[00:02:56] 765 |                 Mode::ToolTest => { },
[00:02:56]     |                 ^^^^^^^^^^^^^^ variant not found in `Mode`
[00:02:56]     | 
[00:02:56]    ::: bootstrap/lib.rs:319:1
[00:02:56]     |
[00:02:56] 319 | pub enum Mode {
[00:02:56]     | ------------- variant `ToolTest` not found here
[00:02:56] error: aborting due to previous error
[00:02:56] 
[00:02:56] For more information about this error, try `rustc --explain E0599`.
[00:02:56] For more information about this error, try `rustc --explain E0599`.
[00:02:56] error: Could not compile `bootstrap`.
[00:02:56] To learn more, run the command again with --verbose.
[00:02:56] failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
[00:02:56] Build completed unsuccessfully in 0:00:04
[00:02:56] make: *** [prepare] Error 1
---
travis_time:end:0568e832:start=1531319476021543158,finish=1531319476029387059,duration=7843901
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:08d4ba0e
$ head -30 ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
head: cannot open ‘./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers’ for reading: No such file or directory
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:0e36f778
$ dmesg | grep -i kill
