plain
  Downloading https://files.pythonhosted.org/packages/87/c5/7ed94b700d30534f346bb55408ca8501325840bcdc371628cff10d7ba68d/botocore-1.10.26-py2.py3-none-any.whl (4.2MB)
Collecting pyasn1>=0.1.3 (from rsa<=3.5.0,>=3.1.2->awscli)
/usr/local/lib/python2.7/dist-packages/pip/_vendor/requests/packages/urllib3/util/ssl_.py:122: InsecurePlatformWarning: A true SSLContext object is not available. This prevents urllib3 from configuring SSL appropriately and may cause certain SSL connections to fail. You can upgrade to a newer version of Python to solve this. For more information, see https://urllib3.readthedocs.io/en/latest/security.html#insecureplatformwarning.
  InsecurePlatformWarning
  Downloading https://files.pythonhosted.org/packages/a0/70/2c27740f08e477499ce19eefe05dbcae6f19fdc49e9e82ce4768be0643b9/pyasn1-0.4.3-py2.py3-none-any.whl (72kB)
/usr/local/lib/python2.7/dist-packages/pip/_vendor/requests/packages/urllib3/util/ssl_.py:122: InsecurePlatformWarning: A true SSLContext object is not available. This prevents urllib3 from configuring SSL appropriately and may cause certain SSL connections to fail. You can upgrade to a newer version of Python to solve this. For more information, see https://urllib3.readthedocs.io/en/latest/security.html#insecureplatformwarning.
  InsecurePlatformWarning
  Downloading https://files.pythonhosted.org/packages/2d/99/b2c4e9d5a30f6471e410a146232b4118e697fa3ffc06d6a65efde84debd0/futures-3.2.0-py2-none-any.whl
Collecting jmespath<1.0.0,>=0.7.1 (from botocore==1.10.26->awscli)
---
[00:03:55]     Checking syntax_pos v0.0.0 (file:///checkout/src/libsyntax_pos)
[00:03:56]     Checking rustc_errors v0.0.0 (file:///checkout/src/librustc_errors)
[00:04:11]     Checking proc_macro v0.0.0 (file:///checkout/src/libproc_macro)
[00:04:12]     Checking syntax_ext v0.0.0 (file:///checkout/src/libsyntax_ext)
[00:04:29] error[E0277]: the trait bound `mir::interpret::Relocations: std::cmp::PartialOrd` is not satisfied
[00:04:29]    --> librustc/mir/interpret/mod.rs:345:5
[00:04:29]     |
[00:04:29] 345 |     pub relocations: Relocations,
[00:04:29]     |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^ can't compare `mir::interpret::Relocations` with `mir::interpret::Relocations`
[00:04:29]     |
[00:04:29]     = help: the trait `std::cmp::PartialOrd` is not implemented for `mir::interpret::Relocations`
[00:04:29]     = note: required by `std::cmp::PartialOrd::partial_cmp`
[00:04:29] 
[00:04:29] error[E0369]: binary operation `<` cannot be applied to type `mir::interpret::Relocations`
[00:04:29]    --> librustc/mir/interpret/mod.rs:345:5
[00:04:29]     |
[00:04:29] 345 |     pub relocations: Relocations,
[00:04:29]     |
[00:04:29]     |
[00:04:29]     = note: an implementation of `std::cmp::PartialOrd` might be missing for `mir::interpret::Relocations`
[00:04:29] 
[00:04:29] error[E0369]: binary operation `>` cannot be applied to type `mir::interpret::Relocations`
[00:04:29]    --> librustc/mir/interpret/mod.rs:345:5
[00:04:29]     |
[00:04:29] 345 |     pub relocations: Relocations,
[00:04:29]     |
[00:04:29]     |
[00:04:29]     = note: an implementation of `std::cmp::PartialOrd` might be missing for `mir::interpret::Relocations`
[00:04:29] 
[00:04:29] error[E0277]: the trait bound `mir::interpret::Relocations: std::cmp::Ord` is not satisfied
[00:04:29]    --> librustc/mir/interpret/mod.rs:345:5
[00:04:29]     |
[00:04:29] 345 |     pub relocations: Relocations,
[00:04:29]     |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the trait `std::cmp::Ord` is not implemented for `mir::interpret::Relocations`
[00:04:29]     |
[00:04:29]     = note: required by `std::cmp::Ord::cmp`
[00:04:38] error: aborting due to 4 previous errors
[00:04:38] 
[00:04:38] Some errors occurred: E0277, E0369.
[00:04:38] For more information about an error, try `rustc --explain E0277`.
[00:04:38] For more information about an error, try `rustc --explain E0277`.
[00:04:38] error: Could not compile `rustc`.
[00:04:38] 
[00:04:38] Caused by:
[00:04:38]   process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustc --crate-name rustc librustc/lib.rs --color always --error-format json --crate-type dylib --emit=dep-info,metadata -C prefer-dynamic -C opt-level=3 -C metadata=abe31e1c98d3f369 -C extra-filename=-abe31e1c98d3f369 --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps --target x86_64-unknown-linux-gnu -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/release/deps --extern tempdir=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libtempdir-3e386995dc337d92.rmeta --extern log=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/liblog-908351a74ebbba8d.rmeta --extern jobserver=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libjobserver-94fbc955671d506d.rmeta --extern rustc_data_structures=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_data_structures-6a4a79105f9bd54f.rmeta --extern fmt_macros=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libfmt_macros-357b7a8b98d95f3a.rmeta --extern arena=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libarena-54250918861a699e.rmeta --extern rustc_target=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_target-c0040c81b4505a50.rmeta --extern syntax_pos=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libsyntax_pos-cae04dca295ba694.rmeta --extern syntax=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libsyntax-745f4de0590494cb.rmeta --extern byteorder=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libbyteorder-b8199a574fa70348.rmeta --extern rustc_errors=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_errors-221bca35c0d44663.rmeta --extern lazy_static=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/liblazy_static-dc0342f292e689dd.rmeta --extern bitflags=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libbitflags-11bfc3c679285c78.rmeta --extern serialize=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libserialize-b369f4a8ef58b783.rmeta --extern backtrace=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libbacktrace-41851314beb0e86b.rmeta --extern proc_macro=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libproc_macro-6ff7bacc2b99651c.rmeta --extern flate2=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libflate2-11f45a861d5fea25.rmeta --extern rustc_apfloat=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_apfloat-51a6c30185c022f8.rmeta --extern graphviz=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libgraphviz-d89334909265f444.rmeta -L native=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/build/backtrace-sys-f085762345e9053e/out/.libs -L native=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/build/miniz-sys-c0082fee642cc0bf/out` (exit code: 101)
[00:04:38] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "check" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--color" "always" "--features" " jemalloc" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "--message-format" "json"
[00:04:38] expected success, got: exit code: 101
[00:04:38] thread 'main' panicked at 'cargo must succeed', bootstrap/compile.rs:1091:9
[00:04:38] travis_fold:end:stage0-rustc

[00:04:38] travis_time:end:stage0-rustc:start=1527115693400527851,finish=1527115759265216392,duration=65864688541

