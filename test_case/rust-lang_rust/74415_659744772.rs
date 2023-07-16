
 # Check that a cdylib exports its public #[no_mangle] functions
[ "$(nm -g /d/a/rust/rust/build/x86_64-pc-windows-gnu/test/run-make-fulldeps/symbol-visibility/symbol-visibility/liba_cdylib.dll.a | grep -v _imp__ | grep -c public_c_function_from_cdylib)" -eq "1" ]
make[1]: Leaving directory '/d/a/rust/rust/src/test/run-make-fulldeps/symbol-visibility'

------------------------------------------
stderr:
------------------------------------------
make[1]: *** [Makefile:37: all] Error 1

------------------------------------------



failures:
    [run-make] run-make-fulldeps\symbol-visibility
