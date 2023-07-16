
# halt.o contains symbo `_halt`: just an empty function.
rustc --crate-type=cdylib -C lto -C opt-level=3 main.rs -C link-arg=halt.o
