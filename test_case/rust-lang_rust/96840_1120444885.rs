plain
error: make failed
status: exit status: 2
command: "make"
--- stdout -------------------------------
LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/obtain-borrowck/obtain-borrowck:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib" '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc' --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/obtain-borrowck/obtain-borrowck -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/obtain-borrowck/obtain-borrowck  driver.rs -o ""/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/obtain-borrowck/obtain-borrowck"/driver"
--- stderr -------------------------------
--- stderr -------------------------------
error[E0433]: failed to resolve: could not find `WithOptConstParam` in `ty`
   --> driver.rs:109:13
    |
109 |         ty::WithOptConstParam::unknown(def_id),
    |             ^^^^^^^^^^^^^^^^^ could not find `WithOptConstParam` in `ty`
warning: ignoring --out-dir flag due to -o flag

error: aborting due to previous error; 1 warning emitted


For more information about this error, try `rustc --explain E0433`.
make: *** [Makefile:19: all] Error 1



failures:
