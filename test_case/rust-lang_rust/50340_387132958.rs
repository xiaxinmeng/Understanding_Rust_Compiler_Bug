plain
/usr/local/lib/python2.7/dist-packages/pip/_vendor/requests/packages/urllib3/util/ssl_.py:122: InsecurePlatformWarning: A true SSLContext object is not available. This prevents urllib3 from configuring SSL appropriately and may cause certain SSL connections to fail. You can upgrade to a newer version of Python to solve this. For more information, see https://urllib3.readthedocs.io/en/latest/security.html#insecureplatformwarning.
  InsecurePlatformWarning
/usr/local/lib/python2.7/dist-packages/pip/_vendor/requests/packages/urllib3/util/ssl_.py:122: InsecurePlatformWarning: A true SSLContext object is not available. This prevents urllib3 from configuring SSL appropriately and may cause certain SSL connections to fail. You can upgrade to a newer version of Python to solve this. For more information, see https://urllib3.readthedocs.io/en/latest/security.html#insecureplatformwarning.
  InsecurePlatformWarning
  Retrying (Retry(total=4, connect=None, read=None, redirect=None)) after connection broken by 'ConnectTimeoutError(<pip._vendor.requests.packages.urllib3.connection.VerifiedHTTPSConnection object at 0x7f78a00e6990>, 'Connection to files.pythonhosted.org timed out. (connect timeout=15)')': /packages/e8/d7/9a01c1e73cfceedacef3f8837797fe5a8d93f2b3d7c6129622d0bf7658ad/awscli-1.15.14-py2.py3-none-any.whl
  Retrying (Retry(total=3, connect=None, read=None, redirect=None)) after connection broken by 'ConnectTimeoutError(<pip._vendor.requests.packages.urllib3.connection.VerifiedHTTPSConnection object at 0x7f78a00e6e10>, 'Connection to files.pythonhosted.org timed out. (connect timeout=15)')': /packages/e8/d7/9a01c1e73cfceedacef3f8837797fe5a8d93f2b3d7c6129622d0bf7658ad/awscli-1.15.14-py2.py3-none-any.whl
  Retrying (Retry(total=2, connect=None, read=None, redirect=None)) after connection broken by 'ConnectTimeoutError(<pip._vendor.requests.packages.urllib3.connection.VerifiedHTTPSConnection object at 0x7f789f837090>, 'Connection to files.pythonhosted.org timed out. (connect timeout=15)')': /packages/e8/d7/9a01c1e73cfceedacef3f8837797fe5a8d93f2b3d7c6129622d0bf7658ad/awscli-1.15.14-py2.py3-none-any.whl
  Retrying (Retry(total=1, connect=None, read=None, redirect=None)) after connection broken by 'ConnectTimeoutError(<pip._vendor.requests.packages.urllib3.connection.VerifiedHTTPSConnection object at 0x7f789f8371d0>, 'Connection to files.pythonhosted.org timed out. (connect timeout=15)')': /packages/e8/d7/9a01c1e73cfceedacef3f8837797fe5a8d93f2b3d7c6129622d0bf7658ad/awscli-1.15.14-py2.py3-none-any.whl
  Retrying (Retry(total=0, connect=None, read=None, redirect=None)) after connection broken by 'ConnectTimeoutError(<pip._vendor.requests.packages.urllib3.connection.VerifiedHTTPSConnection object at 0x7f789f837310>, 'Connection to files.pythonhosted.org timed out. (connect timeout=15)')': /packages/e8/d7/9a01c1e73cfceedacef3f8837797fe5a8d93f2b3d7c6129622d0bf7658ad/awscli-1.15.14-py2.py3-none-any.whl
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
ConnectTimeout: HTTPSConnectionPool(host='files.pythonhosted.org', port=443): Max retries exceeded with url: /packages/e8/d7/9a01c1e73cfceedacef3f8837797fe5a8d93f2b3d7c6129622d0bf7658ad/awscli-1.15.14-py2.py3-none-any.whl (Caused by ConnectTimeoutError(<pip._vendor.requests.packages.urllib3.connection.VerifiedHTTPSConnection object at 0x7f789f837410>, 'Connection to files.pythonhosted.org timed out. (connect timeout=15)'))
  InsecurePlatformWarning
/usr/local/lib/python2.7/dist-packages/pip/_vendor/requests/packages/urllib3/util/ssl_.py:122: InsecurePlatformWarning: A true SSLContext object is not available. This prevents urllib3 from configuring SSL appropriately and may cause certain SSL connections to fail. You can upgrade to a newer version of Python to solve this. For more information, see https://urllib3.readthedocs.io/en/latest/security.html#insecureplatformwarning.
  InsecurePlatformWarning
You are using pip version 9.0.1, however version 10.0.1 is available.
---
[00:09:40]    Compiling proc_macro v0.0.0 (file:///checkout/src/libproc_macro)
[00:10:03]    Compiling syntax_ext v0.0.0 (file:///checkout/src/libsyntax_ext)
[00:18:20]    Compiling rustc_mir v0.0.0 (file:///checkout/src/librustc_mir)
[00:18:20]    Compiling rustc_typeck v0.0.0 (file:///checkout/src/librustc_typeck)
[00:18:29] error[E0284]: type annotations required: cannot resolve `<[&[rustc::ty::Predicate<'_>]] as std::slice::SliceConcatExt<_>>::Output == std::vec::Vec<rustc::ty::Predicate<'_>>`
[00:18:29]     --> librustc_typeck/collect.rs:1303:75
[00:18:29]      |
[00:18:29] 1303 |         [&explicit.predicates[..], &tcx.inferred_outlives_of(def_id)[..]].concat()
[00:18:29] 
[00:18:30] error: aborting due to previous error
[00:18:30] 
[00:18:30] For more information about this error, try `rustc --explain E0284`.
[00:18:30] For more information about this error, try `rustc --explain E0284`.
[00:18:30] error: Could not compile `rustc_typeck`.
[00:18:30] 
[00:18:30] Caused by:
[00:18:30]   process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustc --crate-name rustc_typeck librustc_typeck/lib.rs --color always --error-format json --crate-type dylib --emit=dep-info,link -C prefer-dynamic -C opt-level=3 -C metadata=26adf89bf3b1030a -C extra-filename=-26adf89bf3b1030a --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps --target x86_64-unknown-linux-gnu -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/release/deps --extern log=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/liblog-4f0866e958f59455.rlib --extern rustc_errors=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_errors-b789a86e1ab64d11.so --extern arena=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libarena-16776be762f4e8c2.so --extern rustc=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc-6af31fe8ff356c2e.so --extern syntax=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libsyntax-566a8d95e6a18781.so --extern rustc_target=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_target-aed9d8ab86b35123.so --extern rustc_data_structures=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_data_structures-f456f53371aa074c.so --extern fmt_macros=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libfmt_macros-33787dcdac3a4dd2.so --extern rustc_platform_intrinsics=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_platform_intrinsics-c44730a3b9fd432b.so --extern syntax_pos=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libsyntax_pos-b9901acb1e9e6766.so -L native=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/build/backtrace-sys-31a3817325787acc/out/.libs -L native=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/build/miniz-sys-90c24e5cae9d047e/out` (exit code: 101)
[00:22:31] error: build failed
