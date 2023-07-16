
% rustup override set nightly-2021-09-22
info: using existing install for 'nightly-2021-09-22-x86_64-unknown-linux-gnu'
info: override toolchain for '/media/pnkfelix/Rust/rust-90357/objdir-dbgopt/rustc-dwarf-regression-repro' set to 'nightly-2021-09-22-x86_64-unknown-linux-gnu'

  nightly-2021-09-22-x86_64-unknown-linux-gnu unchanged - rustc 1.57.0-nightly (ac2d9fc50 2021-09-21)

% rustup show active-toolchain
nightly-2021-09-22-x86_64-unknown-linux-gnu (directory override for '/media/pnkfelix/Rust/rust-90357/objdir-dbgopt/rustc-dwarf-regression-repro')
% rustc --version
rustc 1.57.0-nightly (ac2d9fc50 2021-09-21)
% cargo clean
% cargo build --release --verbose
   Compiling nodwarfrepro v0.1.0 (/media/pnkfelix/Rust/rust-90357/objdir-dbgopt/rustc-dwarf-regression-repro)
     Running `rustc --crate-name nodwarfrepro --edition=2018 src/lib.rs --error-format=json --json=diagnostic-rendered-ansi --crate-type lib --emit=dep-info,metadata,link -C opt-level=3 -C linker-plugin-lto -C debuginfo=2 -C metadata=f2ccefc4246296b8 -C extra-filename=-f2ccefc4246296b8 --out-dir /media/pnkfelix/Rust/rust-90357/objdir-dbgopt/rustc-dwarf-regression-repro/target/release/deps -L dependency=/media/pnkfelix/Rust/rust-90357/objdir-dbgopt/rustc-dwarf-regression-repro/target/release/deps`
     Running `rustc --crate-name nodwarfrepro --edition=2018 src/main.rs --error-format=json --json=diagnostic-rendered-ansi --crate-type bin --emit=dep-info,link -C opt-level=3 -C lto -C debuginfo=2 -C metadata=f4b2592c51b39d90 -C extra-filename=-f4b2592c51b39d90 --out-dir /media/pnkfelix/Rust/rust-90357/objdir-dbgopt/rustc-dwarf-regression-repro/target/release/deps -L dependency=/media/pnkfelix/Rust/rust-90357/objdir-dbgopt/rustc-dwarf-regression-repro/target/release/deps --extern nodwarfrepro=/media/pnkfelix/Rust/rust-90357/objdir-dbgopt/rustc-dwarf-regression-repro/target/release/deps/libnodwarfrepro-f2ccefc4246296b8.rlib`
    Finished release [optimized + debuginfo] target(s) in 2.78s
% dwarfdump target/release/nodwarfrepro | grep THE_STATIC
                        DW_AT_name                  THE_STATIC
                        DW_AT_linkage_name          _ZN12nodwarfrepro10THE_STATIC17he3077d0fbb05bed3E
global die-in-sect 0x00000089, cu-in-sect 0x00000071, die-in-cu 0x00000023, cu-header-in-sect 0x00000066 'THE_STATIC'
name at offset 0x000000d1, length   10 is 'THE_STATIC'
name at offset 0x00000102, length   49 is '_ZN12nodwarfrepro10THE_STATIC17he3077d0fbb05bed3E'
% 
