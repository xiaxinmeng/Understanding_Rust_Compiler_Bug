rust
pub fn foo3(x: usize) -> u32 {
    static BAR: [u32; 10] = [0,10,20,30,40,50,60,70,80,90];
    BAR[x % 10]
}

pub fn foo4(x: usize) -> u32 {
    let BAR: &[u32; 10] = &[0,10,20,30,40,50,60,70,80,90];
    BAR[x % 10]
}
