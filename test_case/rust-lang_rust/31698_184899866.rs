
RUST_BACKTRACE=1 rustc -O -g --cfg disable_float -Z no-landing-pads -C soft-float -C relocation-model=pic -C linker=x86_64-efi-pe-ld --target=target.json --out-dir libs --crate-type=rlib --crate-name=uefi --emit=link -L crate=libs ext/libuefi/src/lib.rs
