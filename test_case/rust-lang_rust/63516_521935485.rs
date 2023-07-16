
let addr:u32 = 0x1A0;
    let addr_uc:u32 = addr | 0xC000_0000; // will be 0xC000_01A0
    let channel:u32 = 3;
    let data:u32 = (addr_uc & 0xFFFF_FFF0) | (channel & 0xF); // will be 0xC000_01A3
    let data_opt:u32 = addr_uc | 0xC000_0008; // will be 0xC000_01A8
    
    println!("data:{:X}, opt:{:X}", data, data_opt);
