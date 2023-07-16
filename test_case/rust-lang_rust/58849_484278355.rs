
james@archx1c6g ➜  /tmp git clone https://github.com/nrf-rs/nrf52832-pac
Cloning into 'nrf52832-pac'...
james@archx1c6g ➜  /tmp cd nrf52832-pac

james@archx1c6g ➜  nrf52832-pac git:(master) time cargo +1.33.0 doc --open
   Compiling semver-parser v0.7.0
   ...
 Documenting nrf52832-pac v0.6.0 (/tmp/nrf52832-pac)
    Finished dev [unoptimized + debuginfo] target(s) in 30.98s
     Opening /tmp/nrf52832-pac/target/doc/nrf52832_pac/index.html
cargo +1.33.0 doc --open  34.23s user 2.07s system 116% cpu 31.205 total

james@archx1c6g ➜  nrf52832-pac git:(master) ✗ cargo clean

james@archx1c6g ➜  nrf52832-pac git:(master) ✗ time cargo +1.34.0 doc --open
   Compiling semver-parser v0.7.0
   ...
    Finished dev [unoptimized + debuginfo] target(s) in 1m 39s
     Opening /tmp/nrf52832-pac/target/doc/nrf52832_pac/index.html
cargo +1.34.0 doc --open  102.90s user 1.71s system 104% cpu 1:39.75 total
