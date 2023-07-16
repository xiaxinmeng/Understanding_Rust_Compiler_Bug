plain
configure: rust.debug-assertions := True
configure: rust.overflow-checks := True
configure: llvm.assertions      := True
configure: dist.missing-tools   := True
configure: build.configure-args := ['--enable-sccache', '--disable-manage-submodu ...
configure: writing `config.toml` in current directory
configure: 
configure: run `python /checkout/x.py --help`
configure: 
---
* highest error code: E0786
Found 502 error codes
Found 0 error codes with no tests
Done!
tidy error: /checkout/src/librustdoc/html/static/css/rustdoc.css:865: CSS files use tabs for indent
tidy error: /checkout/src/librustdoc/html/static/css/rustdoc.css:866: CSS files use tabs for indent
tidy error: /checkout/src/librustdoc/html/static/css/rustdoc.css:867: CSS files use tabs for indent
tidy error: /checkout/src/librustdoc/html/static/css/rustdoc.css:869: CSS files use tabs for indent
tidy error: /checkout/src/librustdoc/html/static/css/rustdoc.css:870: CSS files use tabs for indent
tidy error: /checkout/src/librustdoc/html/static/css/rustdoc.css:872: CSS files use tabs for indent
tidy error: /checkout/src/librustdoc/html/static/css/rustdoc.css:874: CSS files use tabs for indent
tidy error: /checkout/src/librustdoc/html/static/css/rustdoc.css:875: CSS files use tabs for indent
tidy error: /checkout/src/librustdoc/html/static/css/rustdoc.css:876: CSS files use tabs for indent
tidy error: /checkout/src/librustdoc/html/static/css/rustdoc.css:877: CSS files use tabs for indent
tidy error: /checkout/src/librustdoc/html/static/css/rustdoc.css:878: CSS files use tabs for indent
tidy error: /checkout/src/librustdoc/html/static/css/rustdoc.css:879: CSS files use tabs for indent
tidy error: /checkout/src/librustdoc/html/static/css/rustdoc.css:881: CSS files use tabs for indent
tidy error: /checkout/src/librustdoc/html/static/css/rustdoc.css:882: CSS files use tabs for indent
some tidy checks failed



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/rust-tidy" "/checkout" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "/checkout/obj/build" "16"


Build completed unsuccessfully in 0:00:12
