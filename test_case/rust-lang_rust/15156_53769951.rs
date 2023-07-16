
rustc -Z no-landing-pads --cfg cfg_mcu_has_spi --cfg mcu_lpc17xx --cfg arch_cortex_m3 --target thumbv7m-linux-eabi -Ctarget-cpu=cortex-m3 -C relocation_model=static --debuginfo=1 -Z lto  --emit obj  -L /Users/farcaller/src/zinc/build  -o /Users/farcaller/src/zinc/build/intermediate/test/app_test.o  /Users/farcaller/src/zinc/apps/app_test.rs
Assertion failed: (DISubprogram(Scope).describes(MF->getFunction())), function getOrCreateRegularScope, file /Users/rustbuild/src/rust-buildbot/slave/nightly-mac/build/src/llvm/lib/CodeGen/LexicalScopes.cpp, line 179.
