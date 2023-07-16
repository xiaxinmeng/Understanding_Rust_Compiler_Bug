 rust
pub unsafe fn inb(port: u16) -> u8 {                                              
    let ret;                                                                      
    asm!("inb $1, $0" : "={ax}"(ret) : "{dx}"(port) :: "(eax),(edx)" : "volatile")
    return ret;                                                                   
}                                                                                 
