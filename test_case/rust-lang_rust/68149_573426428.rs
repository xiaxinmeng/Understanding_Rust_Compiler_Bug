sh
./x.py test --stage 1 src/test/ui --test-args mytest
rm -rf build/x86_64-pc-windows-gnu/{stage1*,stage0-sysroot}
./x.py test --stage 1 src/test/ui --test-args mytest
./x.py test --stage 1 src/test/ui --test-args mytest -vvvvvv
