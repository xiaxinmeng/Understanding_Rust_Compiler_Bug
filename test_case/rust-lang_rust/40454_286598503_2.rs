rust
unsafe fn time(swap: fn(&mut [u8; ARR_LENGTH], &mut [u8; ARR_LENGTH])) -> u64 {
    let mut x: [u8; ARR_LENGTH] = [42; ARR_LENGTH];
    let mut y: [u8; ARR_LENGTH] = [69; ARR_LENGTH];
    asm!("cpuid
          cpuid
          cpuid
          cpuid
          cpuid" 
          :
          :
          : "rax", "rbx", "rcx", "rdx" 
          : "volatile");
    let mut min_diff = !0;
    let mut max_diff = 0;
    let mut i = 0;
    while i < 80 {
        let before_high: u32;
        let before_low: u32;
        let after_high: u32;
        let after_low: u32;
        asm!("cpuid
              rdtsc
              movl %edx, $0
              movl %eax, $1"
              : "=r" (before_high), "=r" (before_low)
              :
              : "rax", "rbx", "rcx", "rdx"
              : "volatile" );
        swap(&mut x, &mut y);
        asm!("rdtscp
              movl %edx, $0
              movl %eax, $1
              cpuid"
              : "=r" (after_high), "=r" (after_low)
              :
              : "rax", "rbx", "rcx", "rdx"
              : "volatile" );
        asm!("" :: "r"(&x), "r"(&y));
        let diff = ((after_high as u64 >> 32) | after_low as u64) - ((before_high as u64 >> 32) | before_low as u64);
        if diff < min_diff { min_diff = diff; }
        if diff > max_diff { max_diff = diff; }
        i += 1;
    }

    return min_diff;
}

fn noop(_: &mut [u8; ARR_LENGTH], _: &mut [u8; ARR_LENGTH]) {}

fn main() {
    unsafe {
        let min_timing = time(noop);
        let min_execution_time_16 = time(swap_array_16);
        let min_execution_time_32 = time(swap_array_32);
        let min_execution_time_64 = time(swap_array_64);
        let min_execution_time_u64x4 = time(swap_array_u64x4);
        let min_execution_time_u64x8 = time(swap_array_u64x8);
        let min_execution_time_u64x16 = time(swap_array_u64x16);
        println!("arr length {}", ARR_LENGTH);
        println!("min timing {}", min_timing);
        println!("16_bytes {}", min_execution_time_16 - min_timing);
        println!("32_bytes {}", min_execution_time_32 - min_timing);
        println!("64_bytes {}", min_execution_time_64 - min_timing);
        println!("simd_u64x4 {}", min_execution_time_u64x4 - min_timing);
        println!("simd_u64x8 {}", min_execution_time_u64x8 - min_timing);
        println!("simd_u64x16 {}", min_execution_time_u64x16 - min_timing);
    }
}
