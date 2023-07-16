shell
rustc +nightly ./main.rs -C opt-level=1 -C panic=abort -C codegen-units=1
./main
# [1]    619598 illegal hardware instruction (core dumped)  ./main
