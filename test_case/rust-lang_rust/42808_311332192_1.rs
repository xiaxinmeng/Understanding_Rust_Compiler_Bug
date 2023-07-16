
#!/bin/sh # vecel.sh

function f() {
    OUT=vecel.$(rustc --version | sed -e 's/(//' -e 's/)//' | cut -d ' ' -f 4).bin ; rustc --version && rustc ~/Dev/Rust/vecel.rs -o $OUT && strip $OUT
}

for day in 19 20 21 22 23 24 25 26 27; do
    rustup default nightly-2017-06-$day;
    f;
done

CHAN=stable;  rustc +$CHAN ~/Dev/Rust/vecel.rs -o vecel.$CHAN.bin && strip vecel.$CHAN.bin
CHAN=beta;    rustc +$CHAN ~/Dev/Rust/vecel.rs -o vecel.$CHAN.bin && strip vecel.$CHAN.bin
CHAN=nightly; rustc +$CHAN ~/Dev/Rust/vecel.rs -o vecel.$CHAN.bin && strip vecel.$CHAN.bin

ls -alh /tmp/vecel.*.bin
