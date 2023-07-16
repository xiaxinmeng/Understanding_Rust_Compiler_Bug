plain
 Documenting rustdoc v0.0.0 (/checkout/src/librustdoc)
error: unresolved link to `Attributes::links`
   --> src/librustdoc/clean/types.rs:647:55
    |
647 | /// This link will be turned into a rendered link by [`Attributes::links`]
    |                                                       ^^^^^^^^^^^^^^^^^^^ the struct `Attributes` has no field or associated item named `links`
    |
    = note: `-D rustdoc::broken-intra-doc-links` implied by `-D warnings`
error: aborting due to previous error

error: could not document `rustdoc`


Caused by:
  process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustdoc --edition=2018 --crate-type lib --crate-name rustdoc src/librustdoc/lib.rs --target x86_64-unknown-linux-gnu -o /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/doc --error-format=json --json=diagnostic-rendered-ansi -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps --extern arrayvec=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libarrayvec-5260d9dfc7ee3b4d.rmeta --extern itertools=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libitertools-726259c16738a68a.rmeta --extern minifier=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libminifier-29a084d5910347a0.rmeta --extern pulldown_cmark=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libpulldown_cmark-67c1a71e2d914f10.rmeta --extern regex=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libregex-386ccf60cbe13993.rmeta --extern rayon=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/librustc_rayon-baf81608c9760064.rmeta --extern rustdoc_json_types=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/librustdoc_json_types-cbdb82e0a4d9c523.rmeta --extern serde=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libserde-0d5588fb009fc3df.rmeta --extern serde_json=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libserde_json-a5b21ff8350654ff.rmeta --extern smallvec=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libsmallvec-b327c92b86f0e10f.rmeta --extern tempfile=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libtempfile-ccdd67712517b8d3.rmeta --extern tracing=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libtracing-3548bab3e2d14894.rmeta --extern tracing_subscriber=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libtracing_subscriber-a0811620352b9933.rmeta --extern tracing_tree=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libtracing_tree-1ab936cbb7d8ebd2.rmeta -Dwarnings '-Wrustdoc::invalid_codeblock_attributes' --crate-version '1.53.0-nightly
  (687f724d5
  2021-04-04)' --document-private-items --enable-index-page -Zunstable-options` (exit code: 1)


command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "doc" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--locked" "--color" "always" "--manifest-path" "/checkout/src/tools/rustdoc/Cargo.toml" "--no-deps" "-p" "rustdoc" "-p" "rustdoc-json-types"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap dist --host x86_64-unknown-linux-gnu --target x86_64-unknown-linux-gnu --include-default-paths src/tools/build-manifest --rust-profile-use=/tmp/rustc-pgo.profdata
Build completed unsuccessfully in 0:08:21
