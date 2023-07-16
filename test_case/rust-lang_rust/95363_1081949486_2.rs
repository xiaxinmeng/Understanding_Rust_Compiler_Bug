ld
.foo : {
    bar = .;
    KEEP(*(.foo));
}
