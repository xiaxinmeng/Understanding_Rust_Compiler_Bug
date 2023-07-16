
% git remote -v
origin	git@github.com:tock/tock.git (fetch)
origin	git@github.com:tock/tock.git (push)
% git log -1 --format=oneline
d423f7e969972e8cd662a37cd13c4fbf2b0a6ee9 (HEAD -> master, origin/staging, origin/master, origin/HEAD) Merge #1458
% git status
On branch master
Your branch is up to date with 'origin/master'.

nothing to commit, working tree clean
% cd boards/arty-e21/
% RUSTFLAGS="-C link-arg=-Tlayout.ld -C linker=rust-lld -C linker-flavor=ld.lld -C relocation-model=dynamic-no-pic -C link-arg=-zmax-page-size=512" cargo +nightly-2019-10-17 build  --target riscv32imac-unknown-none-elf 
   Compiling tock-registers v0.4.1 (/Users/felixklock/Dev/Mozilla/issue65774/tock/libraries/tock-register-interface)
   Compiling tock-cells v0.1.0 (/Users/felixklock/Dev/Mozilla/issue65774/tock/libraries/tock-cells)
   Compiling tock_rt0 v0.1.0 (/Users/felixklock/Dev/Mozilla/issue65774/tock/libraries/tock-rt0)
   Compiling enum_primitive v0.1.0 (/Users/felixklock/Dev/Mozilla/issue65774/tock/libraries/enum_primitive)
   Compiling arty-e21 v0.1.0 (/Users/felixklock/Dev/Mozilla/issue65774/tock/boards/arty-e21)
   Compiling kernel v0.1.0 (/Users/felixklock/Dev/Mozilla/issue65774/tock/kernel)
   Compiling riscv-csr v0.1.0 (/Users/felixklock/Dev/Mozilla/issue65774/tock/libraries/riscv-csr)
   Compiling rv32i v0.1.0 (/Users/felixklock/Dev/Mozilla/issue65774/tock/arch/rv32i)
   Compiling capsules v0.1.0 (/Users/felixklock/Dev/Mozilla/issue65774/tock/capsules)
   Compiling sifive v0.1.0 (/Users/felixklock/Dev/Mozilla/issue65774/tock/chips/sifive)
   Compiling arty_e21 v0.1.0 (/Users/felixklock/Dev/Mozilla/issue65774/tock/chips/arty_e21)
   Compiling components v0.1.0 (/Users/felixklock/Dev/Mozilla/issue65774/tock/boards/components)
    Finished dev [optimized + debuginfo] target(s) in 10.99s
% 
