
% rustup default nightly-2019-01-13 && touch src/lib.rs && touch ../ppu_loop/src/lib.rs
info: using existing install for 'nightly-2019-01-13-x86_64-apple-darwin'
info: default toolchain set to 'nightly-2019-01-13-x86_64-apple-darwin'

  nightly-2019-01-13-x86_64-apple-darwin unchanged - rustc 1.33.0-nightly (75a369c5b 2019-01-12)

% time cargo check
   Compiling ppu_loop v0.1.0 (/Users/fklock/Dev/Mozilla/issue57335/ppu_loop)
    Checking demo v0.1.0 (/Users/fklock/Dev/Mozilla/issue57335/demo)
expansion ppu_loop_impl total_cycles: 17000
    Finished dev [unoptimized + debuginfo] target(s) in 1.57s

real	0m1.616s
user	0m1.109s
sys	0m0.443s
% rustup default nightly-2019-01-14 && touch src/lib.rs && touch ../ppu_loop/src/lib.rs
info: using existing install for 'nightly-2019-01-14-x86_64-apple-darwin'
info: default toolchain set to 'nightly-2019-01-14-x86_64-apple-darwin'

  nightly-2019-01-14-x86_64-apple-darwin unchanged - rustc 1.33.0-nightly (2fadb0a16 2019-01-13)

% time cargo check
   Compiling ppu_loop v0.1.0 (/Users/fklock/Dev/Mozilla/issue57335/ppu_loop)
    Checking demo v0.1.0 (/Users/fklock/Dev/Mozilla/issue57335/demo)
expansion ppu_loop_impl total_cycles: 17000
    Finished dev [unoptimized + debuginfo] target(s) in 33.07s

real	0m33.115s
user	0m14.167s
sys	0m18.665s
