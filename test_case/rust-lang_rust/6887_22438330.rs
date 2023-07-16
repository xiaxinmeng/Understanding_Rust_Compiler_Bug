
run-ne: nonexhaust.bin
    ./$<

%.bin: %.rs
    rustc -o $@ $<
