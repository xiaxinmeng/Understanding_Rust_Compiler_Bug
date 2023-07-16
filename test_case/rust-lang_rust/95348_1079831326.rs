plain

running 229 tests
..........i.ii....ii................................................................................ 100/229
.................i...................iiiiiii......i...................iii........................... 200/229
Some tests failed in compiletest suite=run-make-fulldeps mode=run-make host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
...........ii..F.............

---- [run-make] run-make-fulldeps/symbol-visibility stdout ----

error: make failed
error: make failed
status: exit status: 2
command: "make"
--- stdout -------------------------------
LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/symbol-visibility/symbol-visibility:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib" '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc' --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/symbol-visibility/symbol-visibility -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/symbol-visibility/symbol-visibility  -Zshare-generics=no an_rlib.rs
LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/symbol-visibility/symbol-visibility:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib" '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc' --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/symbol-visibility/symbol-visibility -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/symbol-visibility/symbol-visibility  -Zshare-generics=no a_cdylib.rs
LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/symbol-visibility/symbol-visibility:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib" '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc' --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/symbol-visibility/symbol-visibility -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/symbol-visibility/symbol-visibility  -Zshare-generics=no a_rust_dylib.rs
LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/symbol-visibility/symbol-visibility:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib" '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc' --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/symbol-visibility/symbol-visibility -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/symbol-visibility/symbol-visibility  -Zshare-generics=no an_executable.rs
LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/symbol-visibility/symbol-visibility:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib" '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc' --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/symbol-visibility/symbol-visibility -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/symbol-visibility/symbol-visibility  -Zshare-generics=no a_cdylib.rs --crate-name combined_rlib_dylib --crate-type=rlib,cdylib
# Check that a cdylib exports its public #[no_mangle] functions
[ "$(nm -D /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/symbol-visibility/symbol-visibility/liba_cdylib.so | grep -v __imp_ | grep -c public_c_function_from_cdylib)" -eq "1" ]
# Check that a cdylib exports the public #[no_mangle] functions of dependencies
[ "$(nm -D /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/symbol-visibility/symbol-visibility/liba_cdylib.so | grep -v __imp_ | grep -c public_c_function_from_rlib)" -eq "1" ]
# Check that a cdylib DOES NOT export any public Rust functions
[ "$(nm -D /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/symbol-visibility/symbol-visibility/liba_cdylib.so | grep -v __imp_ | grep -c "_ZN.*h.*E\|_R[a-zA-Z0-9_]+")" -eq "0" ]
--- stderr -------------------------------
--- stderr -------------------------------
make: *** [Makefile:41: all] Error 1



failures:
