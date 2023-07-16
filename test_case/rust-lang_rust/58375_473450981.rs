
RUSTFLAGS="-C relocation-model=dynamic-no-pic -C link-dead-code -C opt-level=0 -C debuginfo=2" cargo +nightly build --features bit-set,break-dead-code,byteorder,default,fork,num-traits,quick-error,rand,regex-syntax,rusty-fork,std,tempfile,timeout
