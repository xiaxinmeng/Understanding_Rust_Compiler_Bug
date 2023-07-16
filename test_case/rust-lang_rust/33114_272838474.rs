Rust
extern crate libc;

use std::io;
use std::os::unix::io::AsRawFd;
use std::fs::File;

const _IOC_READ: libc::c_ulong = 2;
const RNDGETENTCNT: libc::c_ulong =
    (4<<29) +
    (4<<16) +
    ((b'R' as libc::c_ulong)<<8) +
    (0x00<<0);
const RNDADDENTROPY: libc::c_ulong =
    (2<<29) +
    (8<<16) +
    ((b'R' as libc::c_ulong)<<8) +
    (0x03<<0);

#[repr(C)]
pub struct RandPoolInfo {
    entropy_count: libc::c_int,
    buf_size: libc::c_int,
    buf: [u32; 4]
}

fn main() {
    main2().unwrap();
}

fn get_entropy_count(file: &File) -> io::Result<libc::c_int> {
    let fd = file.as_raw_fd();
    let mut data = 0;
    let result = unsafe {
        libc::ioctl(fd, RNDGETENTCNT as _, &mut data as *mut _)
    };
    if result != 0 {
        return Err(io::Error::last_os_error());
    }
    Ok(data)
}

fn add_entropy(file: &File, buf: [u32; 4]) -> io::Result<()> {
    let pool = RandPoolInfo {
        entropy_count: 128,
        buf_size: 16,
        buf: buf,
    };

    let fd = file.as_raw_fd();
    let result = unsafe {
        libc::ioctl(fd, RNDADDENTROPY as _, &pool as *const RandPoolInfo)
    };
    if result != 0 {
        return Err(io::Error::last_os_error());
    }

    Ok(())
}

fn main2() -> io::Result<()> {
    let f = File::open("/dev/random")?;

    println!("entropy before: {}", get_entropy_count(&f)?);

    add_entropy(
        &f,
        // chosen by fair dice roll, guaranteed to be random
        // please use 4 fresh random words in production
        [1825268108, 1056140314, 3447110939, 1873895583]
    )?;

    println!("entropy after: {}", get_entropy_count(&f)?);

    Ok(())
}
