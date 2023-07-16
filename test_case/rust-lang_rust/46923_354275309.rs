
RUSTFLAGS=-Ctarget-feature=+avx\ -Ccodegen-units=1 cargo +nightly bench -v m127
test mat_mul_f32::m127             ... bench:     123,960 ns/iter (+/- 8,354)
test mat_mul_f64::m127             ... bench:     194,921 ns/iter (+/- 3,862)
