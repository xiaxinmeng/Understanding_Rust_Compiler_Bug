plain
/usr/local/lib/python2.7/dist-packages/pip/_vendor/requests/packages/urllib3/util/ssl_.py:122: InsecurePlatformWarning: A true SSLContext object is not available. This prevents urllib3 from configuring SSL appropriately and may cause certain SSL connections to fail. You can upgrade to a newer version of Python to solve this. For more information, see https://urllib3.readthedocs.io/en/latest/security.html#insecureplatformwarning.
  InsecurePlatformWarning
/usr/local/lib/python2.7/dist-packages/pip/_vendor/requests/packages/urllib3/util/ssl_.py:122: InsecurePlatformWarning: A true SSLContext object is not available. This prevents urllib3 from configuring SSL appropriately and may cause certain SSL connections to fail. You can upgrade to a newer version of Python to solve this. For more information, see https://urllib3.readthedocs.io/en/latest/security.html#insecureplatformwarning.
  InsecurePlatformWarning
  Retrying (Retry(total=4, connect=None, read=None, redirect=None)) after connection broken by 'ConnectTimeoutError(<pip._vendor.requests.packages.urllib3.connection.VerifiedHTTPSConnection object at 0x7fb1625cd990>, 'Connection to files.pythonhosted.org timed out. (connect timeout=15)')': /packages/e8/d7/9a01c1e73cfceedacef3f8837797fe5a8d93f2b3d7c6129622d0bf7658ad/awscli-1.15.14-py2.py3-none-any.whl
  Retrying (Retry(total=3, connect=None, read=None, redirect=None)) after connection broken by 'ConnectTimeoutError(<pip._vendor.requests.packages.urllib3.connection.VerifiedHTTPSConnection object at 0x7fb1625cde10>, 'Connection to files.pythonhosted.org timed out. (connect timeout=15)')': /packages/e8/d7/9a01c1e73cfceedacef3f8837797fe5a8d93f2b3d7c6129622d0bf7658ad/awscli-1.15.14-py2.py3-none-any.whl
  Retrying (Retry(total=2, connect=None, read=None, redirect=None)) after connection broken by 'ConnectTimeoutError(<pip._vendor.requests.packages.urllib3.connection.VerifiedHTTPSConnection object at 0x7fb161d1e090>, 'Connection to files.pythonhosted.org timed out. (connect timeout=15)')': /packages/e8/d7/9a01c1e73cfceedacef3f8837797fe5a8d93f2b3d7c6129622d0bf7658ad/awscli-1.15.14-py2.py3-none-any.whl
  Retrying (Retry(total=1, connect=None, read=None, redirect=None)) after connection broken by 'ConnectTimeoutError(<pip._vendor.requests.packages.urllib3.connection.VerifiedHTTPSConnection object at 0x7fb161d1e1d0>, 'Connection to files.pythonhosted.org timed out. (connect timeout=15)')': /packages/e8/d7/9a01c1e73cfceedacef3f8837797fe5a8d93f2b3d7c6129622d0bf7658ad/awscli-1.15.14-py2.py3-none-any.whl
  Retrying (Retry(total=0, connect=None, read=None, redirect=None)) after connection broken by 'ConnectTimeoutError(<pip._vendor.requests.packages.urllib3.connection.VerifiedHTTPSConnection object at 0x7fb161d1e310>, 'Connection to files.pythonhosted.org timed out. (connect timeout=15)')': /packages/e8/d7/9a01c1e73cfceedacef3f8837797fe5a8d93f2b3d7c6129622d0bf7658ad/awscli-1.15.14-py2.py3-none-any.whl
Traceback (most recent call last):
  File "/usr/local/lib/python2.7/dist-packages/pip/basecommand.py", line 215, in main
    status = self.run(options, args)
  File "/usr/local/lib/python2.7/dist-packages/pip/commands/install.py", line 335, in run
---
    r = adapter.send(request, **kwargs)
  File "/usr/local/lib/python2.7/dist-packages/pip/_vendor/cachecontrol/adapter.py", line 47, in send
    resp = super(CacheControlAdapter, self).send(request, **kw)
  File "/usr/local/lib/python2.7/dist-packages/pip/_vendor/requests/adapters.py", line 479, in send
    raise ConnectTimeout(e, request=request)
ConnectTimeout: HTTPSConnectionPool(host='files.pythonhosted.org', port=443): Max retries exceeded with url: /packages/e8/d7/9a01c1e73cfceedacef3f8837797fe5a8d93f2b3d7c6129622d0bf7658ad/awscli-1.15.14-py2.py3-none-any.whl (Caused by ConnectTimeoutError(<pip._vendor.requests.packages.urllib3.connection.VerifiedHTTPSConnection object at 0x7fb161d1e410>, 'Connection to files.pythonhosted.org timed out. (connect timeout=15)'))
  InsecurePlatformWarning
/usr/local/lib/python2.7/dist-packages/pip/_vendor/requests/packages/urllib3/util/ssl_.py:122: InsecurePlatformWarning: A true SSLContext object is not available. This prevents urllib3 from configuring SSL appropriately and may cause certain SSL connections to fail. You can upgrade to a newer version of Python to solve this. For more information, see https://urllib3.readthedocs.io/en/latest/security.html#insecureplatformwarning.
  InsecurePlatformWarning
You are using pip version 9.0.1, however version 10.0.1 is available.
---
[00:07:38] 
[00:07:38] error: unused import: `Ty`
[00:07:38]   --> librustc/mir/interpret/mod.rs:20:16
[00:07:38]    |
[00:07:38] 20 | use ty::{self, Ty, TyCtxt};
[00:07:38] 
[00:08:04] error: aborting due to 2 previous errors
[00:08:04] 
[00:08:04] error: Could not compile `rustc`.
[00:08:04] error: Could not compile `rustc`.
[00:08:04] 
[00:08:04] Caused by:
[00:08:04]   process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustc --crate-name rustc librustc/lib.rs --color always --error-format json --crate-type dylib --emit=dep-info,link -C prefer-dynamic -C opt-level=3 -C metadata=6af31fe8ff356c2e -C extra-filename=-6af31fe8ff356c2e --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps --target x86_64-unknown-linux-gnu -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/release/deps --extern rustc_apfloat=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_apfloat-b7c66a9cab3ff5a6.rlib --extern flate2=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libflate2-206e1aadbc3c6b8a.rlib --extern tempdir=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libtempdir-ee923a086d887011.rlib --extern syntax_pos=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libsyntax_pos-b9901acb1e9e6766.so --extern bitflags=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libbitflags-401bddd0d1809e53.rlib --extern arena=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustcnux-gnu/release/deps/libgraphviz-21ce4bd19908f0cc.so --extern serialize=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libserialize-946eff7380f27f57.so --extern serialize=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libserialize-946eff7380f27f57.rlib --extern rustc_data_structures=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_data_structures-f456f53371aa074c.so -L native=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/build/backtrace-sys-31a3817325787acc/out/.libs -L native=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/build/miniz-sys-90c24e5cae9d047e/out` (exit code: 101)
[00:08:04] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "--release" "--locked" "--color" "always" "--features" " jemalloc" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "--message-format" "json"
[00:08:04] expected success, got: exit code: 101
[00:08:04] thread 'main' panicked at 'cargo must succeed', bootstrap/compile.rs:1091:9
[00:08:04] travis_fold:end:stage0-rustc

[00:08:04] travis_time:end:stage0-rustc:start=1525703904578913082,finish=1525704089571455113,duration=184992542031


[00:08:04] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap build
[00:08:04] Build completed unsuccessfully in 0:03:18
[00:08:05] make: *** [all] Error 1
[00:08:05] Makefile:28: recipe for target 'all' failed

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:13ffc56c
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
