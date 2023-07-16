
RUSTFLAGS='-Z unstable-options --check-cfg=values(feature,"f_a","f_b","f_n")' cargo build
