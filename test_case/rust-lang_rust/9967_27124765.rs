
run codegen [x86_64-unknown-linux-gnu]: x86_64-unknown-linux-gnu/stage2/bin/compiletest

running 9 tests
test [codegen] codegen/iterate-over-array.rs                ... metric: clang-codegen-ratio: 0.620253 (+/- 0.001)
test [codegen] codegen/scalar-function-call.rs              ... metric: clang-codegen-ratio: 0.8125 (+/- 0.001)
test [codegen] codegen/single-return-value.rs               ... metric: clang-codegen-ratio: 1.3 (+/- 0.001)
test [codegen] codegen/small-dense-int-switch.rs            ... metric: clang-codegen-ratio: 0.711111 (+/- 0.001)
test [codegen] codegen/stack-alloc-string-slice.rs          ... metric: clang-codegen-ratio: 0.619048 (+/- 0.001)
test [codegen] codegen/static-method-call-multi.rs          ... metric: clang-codegen-ratio: 0.805556 (+/- 0.001)
test [codegen] codegen/static-method-call.rs                ... metric: clang-codegen-ratio: 0.8 (+/- 0.001)
test [codegen] codegen/virtual-method-call-struct-return.rs ... metric: clang-codegen-ratio: 0.83871 (+/- 0.001)
test [codegen] codegen/virtual-method-call.rs               ... metric: clang-codegen-ratio: 1.1 (+/- 0.001)

using metrics ratcher: tmp/check-stage2-T-x86_64-unknown-linux-gnu-H-x86_64-unknown-linux-gnu-codegen-metrics.json
[codegen] codegen/iterate-over-array.rs.clang-codegen-ratio: regressed by 4.26%
result of ratchet: 0 matrics added, 0 removed, 0 improved, 1 regressed, 8 noise
left ratchet file untouched

test result: FAILED. 0 passed; 0 failed; 0 ignored; 9 measured

