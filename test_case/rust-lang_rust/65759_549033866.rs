
ENV WASM_TARGETS=wasm32-unknown-unknown
ENV WASM_SCRIPT python2.7 /checkout/x.py test --target $WASM_TARGETS \
  src/test/run-make \
  src/test/ui \
  src/test/compile-fail \
  src/test/mir-opt \
  src/test/codegen-units \
  src/libcore
