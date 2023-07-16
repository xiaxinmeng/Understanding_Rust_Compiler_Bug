plain
   Compiling difference v2.0.0
   Compiling once_cell v1.7.2
   Compiling expect-test v1.0.1
   Compiling rustdoc v0.0.0 (/checkout/src/librustdoc)
error[E0560]: struct `Markdown<'_>` has no field named `heading_level`
   --> src/librustdoc/html/markdown/tests.rs:157:13
157 |             heading_level: 1,
157 |             heading_level: 1,
    |             ^^^^^^^^^^^^^ `Markdown<'_>` does not have this field
    |
    = note: available fields are: `content`, `links`, `ids`, `error_codes`, `edition` ... and 2 others

error[E0560]: struct `Markdown<'_>` has no field named `id`
   --> src/librustdoc/html/markdown/tests.rs:195:13
195 |             id: map,
    |             ^^ help: a field with a similar name exists: `ids`


error[E0560]: struct `Markdown<'_>` has no field named `heading_level`
   --> src/librustdoc/html/markdown/tests.rs:199:13
199 |             heading_level: 1,
199 |             heading_level: 1,
    |             ^^^^^^^^^^^^^ `Markdown<'_>` does not have this field
    |
    = note: available fields are: `content`, `links`, `ids`, `error_codes`, `edition` ... and 2 others
For more information about this error, try `rustc --explain E0560`.
error: could not compile `rustdoc` due to 3 previous errors



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--locked" "--color" "always" "--manifest-path" "/checkout/src/tools/rustdoc/Cargo.toml" "-p" "rustdoc:0.0.0" "--" "--quiet"


Build completed unsuccessfully in 0:26:09
