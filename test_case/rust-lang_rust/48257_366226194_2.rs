`
RUSTC_BOOTSTRAP=1 RUSTFLAGS="-Z time-passes -C codegen-units=1" cargo +stable  build --release 

real	1m27,657s
user	2m57,814s
sys	0m3,034s

