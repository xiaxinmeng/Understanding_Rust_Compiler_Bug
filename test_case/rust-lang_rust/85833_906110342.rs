plain
configure: rust.channel         := nightly
configure: rust.debug-assertions := True
configure: llvm.assertions      := True
configure: dist.missing-tools   := True
configure: build.configure-args := ['--enable-sccache', '--disable-manage-submodu ...
configure: writing `config.toml` in current directory
configure: 
configure: run `python /checkout/x.py --help`
configure: 
---
* highest error code: E0784
Found 500 error codes
Found 0 error codes with no tests
Done!
tidy error: /checkout/src/librustdoc/html/static/css/rustdoc.css:1965: CSS files use tabs for indent
tidy error: /checkout/src/librustdoc/html/static/css/rustdoc.css:1966: CSS files use tabs for indent
tidy error: /checkout/src/librustdoc/html/static/css/rustdoc.css:2069: CSS files use tabs for indent
tidy error: /checkout/src/librustdoc/html/static/css/rustdoc.css:2074: CSS files use tabs for indent
tidy error: /checkout/src/librustdoc/html/static/css/rustdoc.css:2075: CSS files use tabs for indent
tidy error: /checkout/src/librustdoc/html/static/css/rustdoc.css:2076: CSS files use tabs for indent
tidy error: /checkout/src/librustdoc/html/static/css/rustdoc.css:2077: CSS files use tabs for indent
tidy error: /checkout/src/librustdoc/html/static/css/rustdoc.css:2078: CSS files use tabs for indent
tidy error: /checkout/src/librustdoc/html/static/css/rustdoc.css:2082: CSS files use tabs for indent
tidy error: /checkout/src/librustdoc/html/static/css/rustdoc.css:2086: CSS files use tabs for indent
tidy error: /checkout/src/librustdoc/html/static/css/rustdoc.css:2087: CSS files use tabs for indent
tidy error: /checkout/src/librustdoc/html/static/css/rustdoc.css:2088: CSS files use tabs for indent
tidy error: /checkout/src/librustdoc/html/static/css/rustdoc.css:2092: CSS files use tabs for indent
tidy error: /checkout/src/librustdoc/html/static/css/rustdoc.css:2096: CSS files use tabs for indent
tidy error: /checkout/src/librustdoc/html/static/css/rustdoc.css:2100: CSS files use tabs for indent
tidy error: /checkout/src/librustdoc/html/static/css/rustdoc.css:2104: CSS files use tabs for indent
some tidy checks failed



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/rust-tidy" "/checkout" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "/checkout/obj/build" "16"


Build completed unsuccessfully in 0:00:12
