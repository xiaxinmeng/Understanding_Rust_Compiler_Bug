
root@dea5c1fdc0d5:/usr/src/repro# rustup default nightly-2019-12-09
info: using existing install for 'nightly-2019-12-09-x86_64-unknown-linux-gnu'
info: default toolchain set to 'nightly-2019-12-09-x86_64-unknown-linux-gnu'

  nightly-2019-12-09-x86_64-unknown-linux-gnu unchanged - rustc 1.41.0-nightly (59947fcae 2019-12-08)

root@dea5c1fdc0d5:/usr/src/repro# ./repro-min.sh
   Compiling repro v0.1.0 (/usr/src/repro)
    Finished release [optimized] target(s) in 0.20s
root@dea5c1fdc0d5:/usr/src/repro# rustup default nightly-2019-12-10
info: using existing install for 'nightly-2019-12-10-x86_64-unknown-linux-gnu'
info: default toolchain set to 'nightly-2019-12-10-x86_64-unknown-linux-gnu'

  nightly-2019-12-10-x86_64-unknown-linux-gnu unchanged - rustc 1.41.0-nightly (76a252ea9 2019-12-09)

root@dea5c1fdc0d5:/usr/src/repro# ./repro-min.sh
   Compiling repro v0.1.0 (/usr/src/repro)
    Finished release [optimized] target(s) in 0.20s
ld: target/i686-unknown-linux-gnu/release/librepro.a(std-c3eaafedbae89cb4.std.52ue7rpt-cgu.0.rcgu.o): warning: relocation against `__rust_probestack' in read-only section `.text._ZN3std3sys4unix2fs4copy17h783e034bf468da7dE'
ld: warning: creating a DT_TEXTREL in a shared object
root@dea5c1fdc0d5:/usr/src/repro#
