
use std::arch::asm;
use syscalls::*;

const TILE_BYTES_PER_ROW: u16 = 8; // N (4x due to dword)
const TILE_ROWS_T0: u8 = 3; // M
const TILE_ROWS_T1: u8 = 3; // M
const TILE_ROWS_T2: u8 = 2; //

#[repr(packed)]
struct amx_memory_layout {
    palette: u8, // Leaving those value undefined makes Segmentation fault
    start_row: u8,
    reserved: [u8; 14],
    tiles_bytes_per_row: [u16; 8], // Max availale ie.g. 64 bytes per tile's row
    reserved2: [u16; 8],
    tiles_rows: [u8; 8], // Max availale ie.g. 64 bytes per tile's row
    reserved3: [u8; 8],
}
impl amx_memory_layout {
    fn new() -> Self {
        amx_memory_layout {
            palette: 1,
            start_row: 0,
            reserved: [0; 14],
            tiles_bytes_per_row: [TILE_BYTES_PER_ROW; 8],
            reserved2: [0; 8],
            tiles_rows: [TILE_ROWS_T0, TILE_ROWS_T1, TILE_ROWS_T2, 8, 8, 8, 8, 8],
            reserved3: [0; 8],
        }
    }
}

fn load_amx_config() {
    // lets initialize palette
    let mycfg: [amx_memory_layout; 1] = [amx_memory_layout::new()];
    unsafe {
        asm!(
        "ldtilecfg [{cfg}]",
        cfg = in(reg)  mycfg.as_ptr(),
        )
    }
}

pub unsafe fn bench_amx() {
    const D_NUM_ELEMENTS: usize = TILE_BYTES_PER_ROW as usize * TILE_ROWS_T0 as usize / 4 as usize;
    const S1_NUM_ELEMENTS: usize = TILE_BYTES_PER_ROW as usize * TILE_ROWS_T1 as usize;
    const S2_NUM_ELEMENTS: usize = TILE_BYTES_PER_ROW as usize * TILE_ROWS_T2 as usize;
    let s1buf: [u8; S1_NUM_ELEMENTS] = [1; S1_NUM_ELEMENTS];
    let s2buf: [u8; S2_NUM_ELEMENTS] = [1; S2_NUM_ELEMENTS];
    let dbuf: [u32; D_NUM_ELEMENTS] = [0; D_NUM_ELEMENTS];
    println!("INITIAL OUTPUT BUFFER: {:?}", dbuf);
    println!("FIRST ARG: {:?}", s1buf);
    println!("SECOND ARG: {:?}", s2buf);
    asm!(
        "tilezero tmm3",
        "tileloadd tmm1, [{s1buf}]",
        "tileloadd tmm2, [{s2buf}]",
        "tdpbuud tmm0, tmm1, tmm2",
        "tilestored [{dbuf}], tmm0",
        s1buf = in(reg) s1buf.as_ptr(),
        s2buf = in(reg) s2buf.as_ptr(),
        dbuf = in(reg) dbuf.as_ptr(),
        out("tmm0") _,
        out("tmm1") _,
        out("tmm2") _,
        out("tmm3") _,
    );
    println!("FINAL OUTPUT BUFFER: {:?}", dbuf);
}

fn initialize_amx_if_available() -> bool {
    const ARCH_GET_XCOMP_PERM: usize = 0x1022;
    const ARCH_REQ_XCOMP_PERM: usize = 0x1023;
    const XFEATURE_XTILECFG: usize = 17;
    const XFEATURE_XTILEDATA: usize = 18;
    const XFEATURE_MASK_XTILEDATA: usize = 1 << XFEATURE_XTILEDATA;
    const XFEATURE_MASK_XTILECFG: usize = 1 << XFEATURE_XTILECFG;
    const XFEATURE_MASK_XTILE: usize = XFEATURE_MASK_XTILECFG | XFEATURE_MASK_XTILEDATA;

    // request kernel to allocate larger signal-stack sizes, so the
    // amx state can be saved (via XSAVE) when a signal arrives
    unsafe {
        // or can get needed size from C++ getauxval(AT_MINSIGSTKSZ) + SIGSTKZ?
        let size = 1024 * 1024;
        let st_mem = libc::malloc(size);
        let new_sig_stack = libc::stack_t {
            ss_flags: 0,
            ss_size: size,
            ss_sp: st_mem,
        };
        let res = libc::sigaltstack(&new_sig_stack, std::ptr::null_mut());
        println!("sigaltstack res = {res:?}, stack addr = {:?}", st_mem);
        if res != 0 {
            panic!("ERROR: Failed to change sigaltstack size");
        }
    }

    let bitmask: [usize; 1] = [0; 1];
    let mut status: usize = 0;
    unsafe {
        let maybe_status = syscall!(Sysno::arch_prctl, ARCH_GET_XCOMP_PERM, bitmask.as_ptr());
        match maybe_status {
            Ok(s) => status = s,
            Err(_) => {
                println!("AMX not supported!");
                return false;
            }
        }
    }

    if (bitmask[0] & XFEATURE_MASK_XTILEDATA) != 0 {
        return true;
    }

    unsafe {
        let maybe_status = syscall!(Sysno::arch_prctl, ARCH_REQ_XCOMP_PERM, XFEATURE_XTILEDATA);
        match maybe_status {
            Ok(s) => status = s,
            Err(err) => {
                println!("AMX Error: XFEATURE_XTILEDATA setup is failed, TMUL usage is not allowed! Error: {}",err);
                return false;
            }
        }
    }
    unsafe {
        status = syscall!(Sysno::arch_prctl, ARCH_GET_XCOMP_PERM, bitmask.as_ptr())
            .expect("Error: ARCH_PRCTL syscall failed!");
    }
    if status != 0 || ((bitmask[0] & XFEATURE_MASK_XTILEDATA) == 0) {
        println!("AMX not supported!");
        return false;
    }

    // XFEATURE_XTILEDATA set successfully, TMUL usage is allowed
    true
}

fn main() {
    if initialize_amx_if_available() == true {
        println!("Success: AMX Enabled!");
        println!("Configuring TMUL tiles:");
        load_amx_config();
        println!("Running dummy dot products");
        unsafe {
            bench_amx();
        }
        println!("Success!");
    } else {
        println!("ERROR: Could not enable AMX!");
    }
}


