plain
21 | extern crate lazy_static;
   | ^^^^^^^^^^^^^^^^^^^^^^^^^
   |
   = note: candidates:
           crate `lazy_static`: /checkout/obj/build/aarch64-unknown-linux-gnu/stage2/lib/rustlib/aarch64-unknown-linux-gnu/lib/liblazy_static-5edc694793a8d129.rlib
error: could not document `rustdoc`

Caused by:
Caused by:
  process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustdoc --edition=2018 --crate-type lib --crate-name rustdoc src/librustdoc/lib.rs --target aarch64-unknown-linux-gnu -o /checkout/obj/build/aarch64-unknown-linux-gnu/stage2-tools/aarch64-unknown-linux-gnu/doc --error-format=json --json=diagnostic-rendered-ansi -L dependency=/checkout/obj/build/aarch64-unknown-linux-gnu/stage2-tools/aarch64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/aarch64-unknown-linux-gnu/stage2-tools/release/deps --extern arrayvec=/checkout/obj/build/aarch64-unknown-linux-gnu/stage2-tools/aarch64-unknown-linux-gnu/release/deps/libarrayvec-74e7b83e22318b98.rmeta --extern itertools=/checkout/obj/build/aarch64-unknown-linux-gnu/stage2-tools/aarch64-unknown-linux-gnu/release/deps/libitertools-e5cf5047fee63d6c.rmeta --extern minifier=/checkout/obj/build/aarch64-unknown-linux-gnu/stage2-tools/aarch64-unknown-linux-gnu/release/deps/libminifier-d0c713342daf61ed.rmeta --extern pulldown_cmark=/checkout/obj/build/aarch64-unknown-linux-gnu/stage2-tools/aarch64-unknown-linux-gnu/release/deps/libpulldown_cmark-92fa2d8fe961a2fd.rmeta --extern regex=/checkout/obj/build/aarch64-unknown-linux-gnu/stage2-tools/aarch64-unknown-linux-gnu/release/deps/libregex-eb57b9492b1b9be7.rmeta --extern rayon=/checkout/obj/build/aarch64-unknown-linux-gnu/stage2-tools/aarch64-unknown-linux-gnu/release/deps/librustc_rayon-a207d70674577c89.rmeta --extern rustdoc_json_types=/checkout/obj/build/aarch64-unknown-linux-gnu/stage2-tools/aarch64-unknown-linux-gnu/release/deps/librustdoc_json_types-6f103161ec2f4597.rmeta --extern serde=/checkout/obj/build/aarch64-unknown-linux-gnu/stage2-tools/aarch64-unknown-linux-gnu/release/deps/libserde-faeaf20e2146bccc.rmeta --extern serde_json=/checkout/obj/build/aarch64-unknown-linux-gnu/stage2-tools/aarch64-unknown-linux-gnu/release/deps/libserde_json-c35fcd6210132986.rmeta --extern smallvec=/checkout/obj/build/aarch64-unknown-linux-gnu/stage2-tools/aarch64-unknown-linux-gnu/release/deps/libsmallvec-f699cd00900a6f31.rmeta --extern tempfile=/checkout/obj/build/aarch64-unknown-linux-gnu/stage2-tools/aarch64-unknown-linux-gnu/release/deps/libtempfile-202d235419d42e33.rmeta --extern tera=/checkout/obj/build/aarch64-unknown-linux-gnu/stage2-tools/aarch64-unknown-linux-gnu/release/deps/libtera-b479ed69880b8872.rmeta --extern tracing=/checkout/obj/build/aarch64-unknown-linux-gnu/stage2-tools/aarch64-unknown-linux-gnu/release/deps/libtracing-e36af799aea9de12.rmeta --extern tracing_subscriber=/checkout/obj/build/aarch64-unknown-linux-gnu/stage2-tools/aarch64-unknown-linux-gnu/release/deps/libtracing_subscriber-14a20c9573dcc42c.rmeta --extern tracing_tree=/checkout/obj/build/aarch64-unknown-linux-gnu/stage2-tools/aarch64-unknown-linux-gnu/release/deps/libtracing_tree-ac8b3e09346bc6fe.rmeta -Dwarnings '-Wrustdoc::invalid_codeblock_attributes' --crate-version '1.57.0-nightly
  (51431e113
  2021-09-20)' --document-private-items --enable-index-page --show-type-layout -Zunstable-options` (exit status: 1)


command did not execute successfully: "/checkout/obj/build/aarch64-unknown-linux-gnu/stage0/bin/cargo" "doc" "--target" "aarch64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "14" "--release" "--locked" "--color" "always" "--manifest-path" "/checkout/src/tools/rustdoc/Cargo.toml" "-Zskip-rustdoc-fingerprint" "--no-deps" "-p" "rustdoc" "-p" "rustdoc-json-types"


Build completed unsuccessfully in 0:39:25
