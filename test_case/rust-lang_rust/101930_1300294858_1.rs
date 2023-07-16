bash
#!/bin/bash

RUSTFLAGS="-C passes=sancov-module -C llvm-args=-sanitizer-coverage-level=4 -C llvm-args=-sanitizer-coverage-trace-compares -C llvm-args=-sanitizer-coverage-inline-8bit-counters -C llvm-args=-sanitizer-coverage-pc-table -C link-dead-code -Z sanitizer=address -C llvm-args=-sanitizer-coverage-stack-depth -C debug-assertions -C codegen-units=1" cargo build -Zbuild-std --target=x86_64-unknown-linux-gnu
