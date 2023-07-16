plain
error: make failed
status: exit status: 2
command: "make"
--- stdout -------------------------------
LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/emit-shared-files/emit-shared-files:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib" '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc' -L /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib -Z unstable-options --emit=invocation-specific --output /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/emit-shared-files/emit-shared-files/invocation-only --resource-suffix=-xxx --theme y.css --extend-css z.css x.rs
[ -e /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/emit-shared-files/emit-shared-files/invocation-only/search-index-xxx.js ]
[ -e /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/emit-shared-files/emit-shared-files/invocation-only/settings.html ]
[ -e /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/emit-shared-files/emit-shared-files/invocation-only/x/all.html ]
[ -e /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/emit-shared-files/emit-shared-files/invocation-only/x/index.html ]
[ -e /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/emit-shared-files/emit-shared-files/invocation-only/theme-xxx.css ] # generated from z.css
! [ -e /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/emit-shared-files/emit-shared-files/invocation-only/storage-xxx.js ]
! [ -e /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/emit-shared-files/emit-shared-files/invocation-only/SourceSerif4-It.ttf.woff ]
# FIXME: this probably shouldn't have a suffix
[ -e /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/emit-shared-files/emit-shared-files/invocation-only/y-xxx.css ]
# FIXME: this is technically incorrect (see `write_shared`)
! [ -e /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/emit-shared-files/emit-shared-files/invocation-only/main-xxx.js ]
LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/emit-shared-files/emit-shared-files:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib" '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc' -L /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib -Z unstable-options --emit=toolchain-shared-resources --output /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/emit-shared-files/emit-shared-files/toolchain-only --resource-suffix=-xxx --extend-css z.css x.rs
[ -e /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/emit-shared-files/emit-shared-files/toolchain-only/storage-xxx.js ]
! [ -e /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/emit-shared-files/emit-shared-files/toolchain-only/SourceSerif4-It.ttf.woff ]
! [ -e /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/emit-shared-files/emit-shared-files/toolchain-only/search-index-xxx.js ]
! [ -e /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/emit-shared-files/emit-shared-files/toolchain-only/x/index.html ]
! [ -e /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/emit-shared-files/emit-shared-files/toolchain-only/theme.css ]
[ -e /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/emit-shared-files/emit-shared-files/toolchain-only/main-xxx.js ]
! [ -e /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/emit-shared-files/emit-shared-files/toolchain-only/y-xxx.css ]
LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/emit-shared-files/emit-shared-files:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib" '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc' -L /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib -Z unstable-options --emit=toolchain-shared-resources,unversioned-shared-resources --output /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/emit-shared-files/emit-shared-files/all-shared --resource-suffix=-xxx --extend-css z.css x.rs
[ -e /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/emit-shared-files/emit-shared-files/all-shared/storage-xxx.js ]
[ -e /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/emit-shared-files/emit-shared-files/all-shared/SourceSerif4-It.ttf.woff ]
--- stderr -------------------------------
--- stderr -------------------------------
warning: theme file "y.css" is missing CSS rules from the default theme
  = warning: the theme may appear incorrect when loaded
  = warning: the theme may appear incorrect when loaded
  = help: to see what rules are missing, call `rustdoc --check-theme "y.css"`

make: *** [Makefile:38: all-shared] Error 1



failures:
