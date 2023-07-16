`
RUSTC_BOOTSTRAP=1 RUSTFLAGS="-Z time-passes -C codegen-units=1"  cargo +1.23.0  build --release
real	1m31,136s
user	3m5,831s
sys	0m3,018s
