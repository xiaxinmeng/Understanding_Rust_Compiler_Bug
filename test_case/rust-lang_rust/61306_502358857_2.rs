make
RUSTC=/home/glaubitz/stage2/bin/rustc

main: main.rs libtest.a
        $(RUSTC) main.rs -L. -ltest

libtest.a: test.o
        ar rc $@ $<

.PHONY: test

test: main
        ./main
