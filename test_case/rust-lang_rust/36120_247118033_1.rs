 make
all:
    rustc -C link-args="-dynamic-linker $A -L$B -L$C $C/crt1.o $C/crti.o $C/crtn.o" -Z use-lld hello.rs 
    ./hello
