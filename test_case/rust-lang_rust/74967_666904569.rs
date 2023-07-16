plain
   Compiling tempfile v3.1.0
   Compiling serde_derive v1.0.106
   Compiling serde_json v1.0.40
   Compiling rustdoc v0.0.0 (/checkout/src/librustdoc)
error: unused import: `rustc_middle::middle::cstore::CrateStore`
   |
   |
17 | use rustc_middle::middle::cstore::CrateStore;
   |
   |
   = note: `-D unused-imports` implied by `-D warnings`

error[E0599]: no method named `def_path_table` found for reference `&rustc_metadata::creader::CStore` in the current scope
    |
    |
123 |                 self.enter_resolver(|r| r.cstore().def_path_table(crate_num).next_id())
    |                                                    ^^^^^^^^^^^^^^ method not found in `&rustc_metadata::creader::CStore`
error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0599`.


Did not run successfully: exit code: 1
"/checkout/obj/build/x86_64-unknown-linux-gnu/stage1/bin/rustc" "--crate-name" "rustdoc" "--edition=2018" "src/librustdoc/lib.rs" "--error-format=json" "--json=diagnostic-rendered-ansi" "--crate-type" "lib" "--emit=dep-info,metadata,link" "-C" "opt-level=3" "-Cembed-bitcode=no" "-C" "debuginfo=0" "-C" "metadata=c4e25bbdb2cbfbd7" "-C" "extra-filename=-c4e25bbdb2cbfbd7" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-tools/x86_64-unknown-linux-gnu/release/deps" "--target" "x86_64-unknown-linux-gnu" "-C" "linker=clang" "-L" "dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-tools/x86_64-unknown-linux-gnu/release/deps" "-L" "dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-tools/release/deps" "--extern" "itertools=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-tools/x86_64-unknown-linux-gnu/release/deps/libitertools-dd82d5121748193a.rmeta" "--extern" "minifier=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-tools/x86_64-unknown-linux-gnu/release/deps/libminifier-966c1716170dd1ef.rmeta" "--extern" "pulldown_cmark=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-tools/x86_64-unknown-linux-gnu/release/deps/libpulldown_cmark-56267caff99f6f9f.rmeta" "--extern" "rayon=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-tools/x86_64-unknown-linux-gnu/release/deps/librustc_rayon-66f8d44f6c3978ea.rmeta" "--extern" "serde=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-tools/x86_64-unknown-linux-gnu/release/deps/libserde-13a0e25aeb4de8b3.rmeta" "--extern" "serde_json=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-tools/x86_64-unknown-linux-gnu/release/deps/libserde_json-fb1ef14d4c222746.rmeta" "--extern" "tempfile=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-tools/x86_64-unknown-linux-gnu/release/deps/libtempfile-12144885b7f5530c.rmeta" "-Zmacro-backtrace" "-Clink-args=-Wl,-rpath,$ORIGIN/../lib" "-Zbinary-dep-depinfo" "-Wrust_2018_idioms" "-Wunused_lifetimes" "-Dwarnings" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage1" "--remap-path-prefix" "/checkout=/rustc/efc44d114215b797e4e501a021a02c4ab500aea0"
error: could not compile `rustdoc`.

To learn more, run the command again with --verbose.

---
== clock drift check ==
  local time: Fri Jul 31 04:06:42 UTC 2020
  network time: Fri, 31 Jul 2020 04:06:42 GMT
== end clock drift check ==
##[error]Process completed with exit code 1.
Terminate orphan process: pid (4713) (python)
