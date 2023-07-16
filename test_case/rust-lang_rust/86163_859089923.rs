
$ cat franta.rs 
#[derive(Debug)]
#[no_mangle]
#[repr(C)]
pub struct Franta {
        a:f32,
        b:f32,
        c:f32,
        d:f32,
}

#[no_mangle]
pub extern "C" fn funkce(arg: Franta, i: i32)
{
    println!("XXX start {:?} {}", arg, i);
}

$ cat testfranta.c 
#include <stdio.h>
struct Franta {
        float a;
        float b;
        float c;
        float d;
};

void
funkce(struct Franta a, int b);

int main() {
        struct Franta a = { 1, 2.0, 2.1, 2.2};
        funkce(a, 3);
        return 0;
}

rustc  --crate-type dylib franta.rs 
gcc ./testfranta.c ./libfranta.so 

$ file a.out 
a.out:          ELF 64-bit MSB executable SPARCV9 Version 1, UltraSPARC3
Extensions Required, dynamically linked, not stripped

$ ./a.out 
XXX start Franta { a: 2.0, b: 2.2, c: NaN, d: NaN } 2136736064
