sh
> dd if=/dev/urandom of=src/out bs=500M count=1
...
> cargo clean && /usr/bin/time -f 'mem %M, user %Us' cargo +nightly build
   Compiling big_inc v0.1.0 (/tmp/big_inc)
    Finished dev [unoptimized + debuginfo] target(s) in 10.52s
mem 4265648, user 9.55s
> cargo clean && /usr/bin/time -f 'mem %M, user %Us' cargo +stage1 build
   Compiling big_inc v0.1.0 (/tmp/big_inc)
    Finished dev [unoptimized + debuginfo] target(s) in 3.39s
mem 2755012, user 2.70s
