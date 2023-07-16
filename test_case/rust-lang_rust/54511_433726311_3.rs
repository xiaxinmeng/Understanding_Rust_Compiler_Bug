 console
$ cargo objdump --target thumbv7m-none-eabi --release --lib -- -d
foo::foo::he14a1cb0136d5f84:
       0:       00 88   ldrh    r0, [r0]
       2:       70 47   bx      lr
