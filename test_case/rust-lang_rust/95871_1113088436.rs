
rustc lib.rs --crate-name repro --crate-type lib --emit=link --target=thumbv7m-none-eabi -C relocation-model=rwpi
