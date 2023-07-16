 rust
struct signature<'self> {
    pattern   : &'self [u32] 
}

struct signature2<'self> {
    pattern   : &'self str,
}

static test1: signature<'static> =  signature {
    pattern:   &[0x243f6a88u32,0x85a308d3u32,0x13198a2eu32,0x03707344u32,0xa4093822u32,0x299f31d0u32],
};
static test2: signature2<'static> =  signature2 {
    pattern:   "Hello!",
};

static test3: &'static [u32] = &[0x243f6a88u32,0x85a308d3u32,0x13198a2eu32,0x03707344u32,0xa4093822u32,0x299f31d0u32];

fn main() {
    println(fmt!("Static &[u32] field:\t%2u", test1.pattern.len()));
    println(fmt!("Static &str field:\t%2u", test2.pattern.len()));
    println(fmt!("Static &[u32]:\t\t%2u", test3.len()));
}
