bash
cargo bisect-rustc --target=riscv32imac-unknown-none-elf --preserve --start=2021-02-05 --end=2021-05-04 -- rustc -- -C linker-plugin-lto=yes --target=riscv32imac-unknown-none-elf 
