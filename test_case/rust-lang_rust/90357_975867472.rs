
% rustup override set nightly-2021-11-22
info: using existing install for 'nightly-2021-11-22-x86_64-unknown-linux-gnu'
info: override toolchain for '/media/pnkfelix/Rust/rust-90357/objdir-dbgopt/rustc-dwarf-regression-repro' set to 'nightly-2021-11-22-x86_64-unknown-linux-gnu'

  nightly-2021-11-22-x86_64-unknown-linux-gnu unchanged - rustc 1.58.0-nightly (65f3f8b22 2021-11-21)

% rustup show active-toolchain
nightly-2021-11-22-x86_64-unknown-linux-gnu (directory override for '/media/pnkfelix/Rust/rust-90357/objdir-dbgopt/rustc-dwarf-regression-repro')
% rustc --version
rustc 1.58.0-nightly (65f3f8b22 2021-11-21)
% cargo clean
% cargo build --release --verbose
   Compiling nodwarfrepro v0.1.0 (/media/pnkfelix/Rust/rust-90357/objdir-dbgopt/rustc-dwarf-regression-repro)
     Running `rustc --crate-name nodwarfrepro --edition=2018 src/lib.rs --error-format=json --json=diagnostic-rendered-ansi --crate-type lib --emit=dep-info,metadata,link -C opt-level=3 -C linker-plugin-lto -C debuginfo=2 -C metadata=f2ccefc4246296b8 -C extra-filename=-f2ccefc4246296b8 --out-dir /media/pnkfelix/Rust/rust-90357/objdir-dbgopt/rustc-dwarf-regression-repro/target/release/deps -L dependency=/media/pnkfelix/Rust/rust-90357/objdir-dbgopt/rustc-dwarf-regression-repro/target/release/deps`
     Running `rustc --crate-name nodwarfrepro --edition=2018 src/main.rs --error-format=json --json=diagnostic-rendered-ansi --crate-type bin --emit=dep-info,link -C opt-level=3 -C lto -C debuginfo=2 -C metadata=f4b2592c51b39d90 -C extra-filename=-f4b2592c51b39d90 --out-dir /media/pnkfelix/Rust/rust-90357/objdir-dbgopt/rustc-dwarf-regression-repro/target/release/deps -L dependency=/media/pnkfelix/Rust/rust-90357/objdir-dbgopt/rustc-dwarf-regression-repro/target/release/deps --extern nodwarfrepro=/media/pnkfelix/Rust/rust-90357/objdir-dbgopt/rustc-dwarf-regression-repro/target/release/deps/libnodwarfrepro-f2ccefc4246296b8.rlib`
    Finished release [optimized + debuginfo] target(s) in 2.55s
% dwarfdump target/release/nodwarfrepro | grep THE_STATIC
% 
