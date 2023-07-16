plain
/usr/local/lib/python2.7/dist-packages/pip/_vendor/requests/packages/urllib3/util/ssl_.py:122: InsecurePlatformWarning: A true SSLContext object is not available. This prevents urllib3 from configuring SSL appropriately and may cause certain SSL connections to fail. You can upgrade to a newer version of Python to solve this. For more information, see https://urllib3.readthedocs.io/en/latest/security.html#insecureplatformwarning.
  InsecurePlatformWarning
/usr/local/lib/python2.7/dist-packages/pip/_vendor/requests/packages/urllib3/util/ssl_.py:122: InsecurePlatformWarning: A true SSLContext object is not available. This prevents urllib3 from configuring SSL appropriately and may cause certain SSL connections to fail. You can upgrade to a newer version of Python to solve this. For more information, see https://urllib3.readthedocs.io/en/latest/security.html#insecureplatformwarning.
  InsecurePlatformWarning
  Downloading https://files.pythonhosted.org/packages/3f/6c/dbbd5992740649134e597833bea5a95e1fc093a7123e9b8156d838b960e4/awscli-1.15.11-py2.py3-none-any.whl (1.3MB)
/usr/local/lib/python2.7/dist-packages/pip/_vendor/requests/packages/urllib3/util/ssl_.py:122: InsecurePlatformWarning: A true SSLContext object is not available. This prevents urllib3 from configuring SSL appropriately and may cause certain SSL connections to fail. You can upgrade to a newer version of Python to solve this. For more information, see https://urllib3.readthedocs.io/en/latest/security.html#insecureplatformwarning.
  InsecurePlatformWarning
  Downloading https://files.pythonhosted.org/packages/e1/ae/baedc9cb175552e95f3395c43055a6a5e125ae4d48a1d7a924baca83e92e/rsa-3.4.2-py2.py3-none-any.whl (46kB)
Collecting s3transfer<0.2.0,>=0.1.12 (from awscli)
Collecting s3transfer<0.2.0,>=0.1.12 (from awscli)
/usr/local/lib/python2.7/dist-packages/pip/_vendor/requests/packages/urllib3/util/ssl_.py:122: InsecurePlatformWarning: A true SSLContext object is not available. This prevents urllib3 from configuring SSL appropriately and may cause certain SSL connections to fail. You can upgrade to a newer version of Python to solve this. For more information, see https://urllib3.readthedocs.io/en/latest/security.html#insecureplatformwarning.
  InsecurePlatformWarning
  Downloading https://files.pythonhosted.org/packages/d7/14/2a0004d487464d120c9fb85313a75cd3d71a7506955be458eebfe19a6b1d/s3transfer-0.1.13-py2.py3-none-any.whl (59kB)
Collecting botocore==1.10.11 (from awscli)
/usr/local/lib/python2.7/dist-packages/pip/_vendor/requests/packages/urllib3/util/ssl_.py:122: InsecurePlatformWarning: A true SSLContext object is not available. This prevents urllib3 from configuring SSL appropriately and may cause certain SSL connections to fail. You can upgrade to a newer version of Python to solve this. For more information, see https://urllib3.readthedocs.io/en/latest/security.html#insecureplatformwarning.
  InsecurePlatformWarning
  Downloading https://files.pythonhosted.org/packages/15/c0/04ec8aec3cdf7dd4887f2666044550eb3370a4f29668e53519cc7143bdcf/botocore-1.10.11-py2.py3-none-any.whl (4.2MB)
Collecting colorama<=0.3.7,>=0.2.5 (from awscli)
/usr/local/lib/python2.7/dist-packages/pip/_vendor/requests/packages/urllib3/util/ssl_.py:122: InsecurePlatformWarning: A true SSLContext object is not available. This prevents urllib3 from configuring SSL appropriately and may cause certain SSL connections to fail. You can upgrade to a newer version of Python to solve this. For more information, see https://urllib3.readthedocs.io/en/latest/security.html#insecureplatformwarning.
  InsecurePlatformWarning
  Downloading https://files.pythonhosted.org/packages/b7/8e/ddb32ddaabd431813e180ca224e844bab8ad42fbb47ee07553f0ec44cd86/colorama-0.3.7-py2.py3-none-any.whl
---
[00:03:28]    Compiling backtrace v0.3.6
[00:03:42]    Compiling rustc_const_math v0.0.0 (file:///checkout/src/librustc_const_math)
[00:03:42]    Compiling proc_macro v0.0.0 (file:///checkout/src/libproc_macro)
[00:03:43]    Compiling syntax_ext v0.0.0 (file:///checkout/src/libsyntax_ext)
[00:03:56] error[E0599]: no method named `map` found for type `syntax::ast::Symbol` in the current scope
[00:03:56]    --> librustc/hir/check_attr.rs:311:28
[00:03:56]     |
[00:03:56] 311 |             if attr.name().map(|name| name == "used").unwrap_or(false) && target != Target::Static {
[00:03:56]     |
[00:03:56]     |
[00:03:56]     = note: the method `map` exists but the following trait bounds were not satisfied:
[00:03:56]             `&mut syntax::ast::Symbol : std::iter::Iterator`
[00:04:11] error: aborting due to previous error
[00:04:11] 
[00:04:11] For more information about this error, try `rustc --explain E0599`.
[00:04:11] error: Could not compile `rustc`.
[00:04:11] error: Could not compile `rustc`.
[00:04:11] 
[00:04:11] Caused by:
[00:04:11]   process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustc --crate-name rustc librustc/lib.rs --color always --error-format json --crate-type dylib --emit=dep-info,metadata -C prefer-dynamic -C debug-assertions=off -C overflow-checks=on -C metadata=52a1228c3522e167 -C extra-filename=-52a1228c3522e167 --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps --target x86_64-unknown-linux-gnu -C incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/incremental -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/release/deps --extern rustc_target=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_target-c6dc897e75251054.rmeta --extern arena=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libarena-859e7ef7a6cf25eb.rmeta --extern fmt_macros=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libfmt_macros-94fc6139b1ea0bf1.rmeta --extern rustc_const_math=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_const_math-7271f139767937d1.rmeta --extern rustc_errors=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_errors-9fd067218c2c9169.rmeta --extern log=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/liblog-b70225e395c9707a.rmeta --extern syntax_pos=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libsyntax_pos-c47a7799a87f795e.rmeta --extern byteorder=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libbyteorder-d4d3cfeb1fd8466e.rmeta --extern backtrace=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libbacktrace-d2aeb6026a91490a.rmeta --extern proc_macro=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libproc_macro-480fd46fb552224c.rmeta --extern graphviz=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libgraphviz-cc95d68b50a37c45.rmeta --extern jobserver=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libjobserver-1e2ef71544006cb2.rmeta --extern flate2=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libflate2-bb43c515469fb087.rmeta --extern tempdir=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libtempdir-17d2dd8801804113.rmeta --extern bitflags=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libbitflags-b694a4c084250fcb.rmeta --extern lazy_static=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/liblazy_static-c592c0690d0cf21a.rmeta --extern rustc_data_structures=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_data_structures-f94e6b9bfd8025fa.rmeta --extern syntax=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libsyntax-fb68dc1b8e637e51.rmeta --extern rustc_apfloat=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_apfloat-c8ad39352cea22de.rmeta --extern serialize=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libserialize-0a253ecb4797bc67.rmeta -L native=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/build/backtrace-sys-838d7b7b46636e17/out/.libs -L native=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/build/miniz-sys-c12600360c64db1e/out` (exit code: 101)

[00:04:11] travis_time:end:stage0-rustc:start=1525133668603089945,finish=1525133732947676694,duration=64344586749


[00:04:11] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "check" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--color" "always" "--features" " jemalloc" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "--message-format" "json"
[00:04:11] expected success, got: exit code: 101
[00:04:11] thread 'main' panicked at 'cargo must succeed', bootstrap/compile.rs:1091:9
[00:04:11] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap check
[00:04:11] Build completed unsuccessfully in 0:03:44
travis_time:end:206935d2:start=1525133481380663637,finish=1525133733187030231,duration=251806366594

