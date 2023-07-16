plain
status: exit status: 2
command: "make"
--- stdout -------------------------------
#Option taking a number
LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/codegen-options-parsing/codegen-options-parsing:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib" '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc' --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/codegen-options-parsing/codegen-options-parsing -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/codegen-options-parsing/codegen-options-parsing  -C codegen-units dummy.rs 2>&1 | \
 "/checkout/src/etc/cat-and-grep.sh" 'codegen option `codegen-units` requires a number'
[[[ begin stdout ]]]
error: codegen option `codegen-units` requires a number (C codegen-units=<value>)


[[[ end stdout ]]]
LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/codegen-options-parsing/codegen-options-parsing:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib" '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc' --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/codegen-options-parsing/codegen-options-parsing -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/codegen-options-parsing/codegen-options-parsing  -C codegen-units= dummy.rs 2>&1 | \
 "/checkout/src/etc/cat-and-grep.sh" 'incorrect value `` for codegen option `codegen-units` - a number was expected'
[[[ begin stdout ]]]
error: incorrect value `` for codegen option `codegen-units` - a number was expected


[[[ end stdout ]]]
LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/codegen-options-parsing/codegen-options-parsing:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib" '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc' --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/codegen-options-parsing/codegen-options-parsing -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/codegen-options-parsing/codegen-options-parsing  -C codegen-units=foo dummy.rs 2>&1 | \
 "/checkout/src/etc/cat-and-grep.sh" 'incorrect value `foo` for codegen option `codegen-units` - a number was expected'
[[[ begin stdout ]]]
error: incorrect value `foo` for codegen option `codegen-units` - a number was expected


[[[ end stdout ]]]
LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/codegen-options-parsing/codegen-options-parsing:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib" '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc' --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/codegen-options-parsing/codegen-options-parsing -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/codegen-options-parsing/codegen-options-parsing  -C codegen-units=1 dummy.rs
#Option taking a string
LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/codegen-options-parsing/codegen-options-parsing:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib" '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc' --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/codegen-options-parsing/codegen-options-parsing -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/codegen-options-parsing/codegen-options-parsing  -C extra-filename dummy.rs 2>&1 | \
 "/checkout/src/etc/cat-and-grep.sh" 'codegen option `extra-filename` requires a string'
[[[ begin stdout ]]]
error: codegen option `extra-filename` requires a string (C extra-filename=<value>)


[[[ end stdout ]]]
LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/codegen-options-parsing/codegen-options-parsing:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib" '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc' --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/codegen-options-parsing/codegen-options-parsing -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/codegen-options-parsing/codegen-options-parsing  -C extra-filename= dummy.rs 2>&1
LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/codegen-options-parsing/codegen-options-parsing:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib" '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc' --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/codegen-options-parsing/codegen-options-parsing -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/codegen-options-parsing/codegen-options-parsing  -C extra-filename=foo dummy.rs 2>&1
#Option taking no argument
LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/codegen-options-parsing/codegen-options-parsing:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib" '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc' --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/codegen-options-parsing/codegen-options-parsing -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/codegen-options-parsing/codegen-options-parsing  -C lto= dummy.rs 2>&1 | \
 "/checkout/src/etc/cat-and-grep.sh" 'codegen option `lto` - either a boolean (`yes`, `no`, `on`, `off`, etc), `thin`, `fat`, or omitted'
[[[ begin stdout ]]]
error: incorrect value `` for codegen option `lto` - either a boolean (`true`, `false`, `yes`, `no`, `on`, `off`, etc), or a string (`"thin"`, `"fat"`), or omitted was expected


[[[ end stdout ]]]
Error: cannot match: codegen option `lto` - either a boolean (`yes`, `no`, `on`, `off`, etc), `thin`, `fat`, or omitted
--- stderr -------------------------------
--- stderr -------------------------------
make: *** [Makefile:14: all] Error 1



failures:
