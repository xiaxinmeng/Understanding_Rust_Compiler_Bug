plain
    100% |████████████████████████████████| 1.3MB 964kB/s 
Collecting botocore==1.10.45 (from awscli)
/usr/local/lib/python2.7/dist-packages/pip/_vendor/requests/packages/urllib3/util/ssl_.py:122: InsecurePlatformWarning: A true SSLContext object is not available. This prevents urllib3 from configuring SSL appropriately and may cause certain SSL connections to fail. You can upgrade to a newer version of Python to solve this. For more information, see https://urllib3.readthedocs.io/en/latest/security.html#insecureplatformwarning.
  InsecurePlatformWarning
  Downloading https://files.pythonhosted.org/packages/2a/c8/b180fb83fa46d2b56ea059177dc3c00647d622987daf5e7ffbc658555ede/botocore-1.10.45-py2.py3-none-any.whl (4.4MB)
    0% |                                | 10kB 43.2MB/s eta 0:00:01
    0% |▏                               | 20kB 18.6MB/s eta 0:00:01
    0% |▎                               | 30kB 23.5MB/s eta 0:00:01
    0% |▎                               | 40kB 14.2MB/s eta 0:00:01
---
[00:23:15]    Compiling syntax_pos v0.0.0 (file:///checkout/src/libsyntax_pos)
[00:23:18]    Compiling rustc_errors v0.0.0 (file:///checkout/src/librustc_errors)
[00:24:14]    Compiling proc_macro v0.0.0 (file:///checkout/src/libproc_macro)
[00:24:25]    Compiling syntax_ext v0.0.0 (file:///checkout/src/libsyntax_ext)
[00:24:25] error: `?` is not a macro repetition operator
[00:24:25]    --> librustc/lint/mod.rs:111:52
[00:24:25]     |
[00:24:25] 111 |      $lint_edition: expr => $edition_level: ident $(,)?
[00:24:25] 
[00:24:25] 
[00:24:25] error: expected `*` or `+`
[00:24:25]    --> librustc/lint/mod.rs:111:52
[00:24:25]     |
[00:24:25] 111 |      $lint_edition: expr => $edition_level: ident $(,)?
[00:24:25] 
[00:24:25] 
[00:24:25] error: `?` is not a macro repetition operator
[00:24:25]    --> librustc/lint/mod.rs:125:25
[00:24:25]     |
[00:24:25] 125 |     ($( $lint:expr ),* $(,)?) => {{
[00:24:25] 
[00:24:25] 
[00:24:25] error: expected `*` or `+`
[00:24:25]    --> librustc/lint/mod.rs:125:25
[00:24:25]     |
[00:24:25] 125 |     ($( $lint:expr ),* $(,)?) => {{
[00:24:25] 
[00:24:55] error: aborting due to 4 previous errors
[00:24:55] 
[00:24:55] error: Could not compile `rustc`.
[00:24:55] error: Could not compile `rustc`.
[00:24:55] 
[00:24:55] Caused by:
[00:24:55]   process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustc --crate-name rustc librustc/lib.rs --color always --error-format json --crate-type dylib --emit=dep-info,link -C prefer-dynamic -C opt-level=2 -C metadata=976f6e3c1eb047f8 -C extra-filename=-976f6e3c1eb047f8 --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps --target x86_64-unknown-linux-gnu -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/release/deps --extern polonius_engine=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/libpolonius_engine-c8176dd12ca6552b.rlib --extern arena=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/libarena-44e761f8b954b499.so --extern rustc_errors=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_errors-22e1fd34ac64b008.so --extern serialize=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/libserialize-c8132f366b701b34.so --extern serialize=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/libserialize-c8132f366b701b34.rlib --extern graphviz=/checkout/obj/build/x86_64-unknown-linux-gnu/s
