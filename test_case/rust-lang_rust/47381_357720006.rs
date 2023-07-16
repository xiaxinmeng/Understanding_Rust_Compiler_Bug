
../x.py test src/tools/tidy && ../x.py build --incremental --keep-stage 0 --stage 1 src/libstd && ../x.py test --stage 1 src/test/{mir-opt,compile-fail,run-pass}
