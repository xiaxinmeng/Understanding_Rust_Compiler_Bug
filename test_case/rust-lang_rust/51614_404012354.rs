plain
  InsecurePlatformWarning
  Downloading https://files.pythonhosted.org/packages/2d/99/b2c4e9d5a30f6471e410a146232b4118e697fa3ffc06d6a65efde84debd0/futures-3.2.0-py2-none-any.whl
Requirement already satisfied: six>=1.5 in /usr/lib/python2.7/dist-packages (from python-dateutil<3.0.0,>=2.1; python_version >= "2.7"->botocore==1.10.55->awscli)
Building wheels for collected packages: awscli
  Running setup.py bdist_wheel for awscli ... - \ | / - \ | done
Successfully built awscli
Installing collected packages: docutils, jmespath, python-dateutil, botocore, colorama, pyasn1, rsa, futures, s3transfer, awscli
Successfully installed awscli-1.15.56 botocore-1.10.55 colorama-0.3.9 docutils-0.14 futures-3.2.0 jmespath-0.9.3 pyasn1-0.4.3 python-dateutil-2.7.3 rsa-3.4.2 s3transfer-0.1.13
/usr/local/lib/python2.7/dist-packages/pip/_vendor/requests/packages/urllib3/util/ssl_.py:122: InsecurePlatformWarning: A true SSLContext object is not available. This prevents urllib3 from configuring SSL appropriately and may cause certain SSL connections to fail. You can upgrade to a newer version of Python to solve this. For more information, see https://urllib3.readthedocs.io/en/latest/security.html#insecureplatformwarning.
---
[00:07:24]    Compiling syntax_ext v0.0.0 (file:///checkout/src/libsyntax_ext)
[00:07:25] error[E0308]: mismatched types
[00:07:25]   --> libsyntax_ext/concat.rs:55:24
[00:07:25]    |
[00:07:25] 55 |                 if let Some(snippet) = cx.codemap().span_to_snippet(sp) {
[00:07:25]    |                        ^^^^^^^^^^^^^ expected enum `std::result::Result`, found enum `std::option::Option`
[00:07:25]    |
[00:07:25]    = note: expected type `std::result::Result<std::string::String, syntax_pos::SpanSnippetError>`
[00:07:25]               found type `std::option::Option<_>`
[00:07:25] error: aborting due to previous error
[00:07:25] 
[00:07:25] For more information about this error, try `rustc --explain E0308`.
[00:07:25] error: Could not compile `syntax_ext`.
[00:07:25] error: Could not compile `syntax_ext`.
[00:07:25] 
[00:07:25] Caused by:
[00:07:25]   process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustc --crate-name syntax_ext libsyntax_ext/lib.rs --color always --error-format json --crate-type dylib --emit=dep-info,link -C prefer-dynamic -C opt-level=2 -C metadata=e8734f8d1e9b10c1 -C extra-filename=-e8734f8d1e9b10c1 --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps --target x86_64-unknown-linux-gnu -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/release/deps --extern fmt_macros=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libfmt_macros-6ad9c4f0e3eb0853.so --extern proc_macro=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libproc_macro-cf4549c1ea81a6c6.so --extern rustc_data_structures=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_data_structures-dfab6c84d2674220.so --extern rustc_errors=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_errors-fb4c86e007981750.so --extern rustc_target=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_target-e528c05031478194.so --extern syntax=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libsyntax-5f603e9854c9c328.so --extern syntax_pos=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libsyntax_pos-786292eb849f05d6.so` (exit code: 101)
2657056 .
1362896 ./obj
1362864 ./obj/build
771852 ./obj/build/x86_64-unknown-linux-gnu
---
160060 ./.git/modules/src
149120 ./src/llvm-emscripten/test
144236 ./obj/build/bootstrap/debug/incremental
129724 ./obj/build/bootstrap/debug/incremental/bootstrap-2evv84e4ca5z
129720 ./obj/build/bootstrap/debug/incremental/bootstrap-2evv84e4ca5z/s-f2sfye6l7a-3mg9w-1bi9bsw64abjr
97520 ./obj/build/x86_64-unknown-linux-gnu/stage0/lib/rustlib/x86_64-unknown-linux-gnu/codegen-backends
89820 ./src/llvm/test/CodeGen
76376 ./.git/modules/src/tools
70500 ./obj/build/x86_64-unknown-linux-gnu/native
