rust
#![no_std]
#![no_main]

use panic_halt as _;

use arduino_hal::prelude::*;

use core::fmt::Debug;
use ufmt::uWrite;

fn print_hex_arr<S>(tag: &str, serial: &mut S, arr: &[u8])
where
    S: uWrite,
    <S as uWrite>::Error: Debug,
{
    ufmt::uwrite!(serial, "{} = ", tag).unwrap();
    for e in arr.iter() {
        ufmt::uwrite!(serial, "{:02x}", *e).unwrap();
    }
    ufmt::uwrite!(serial, "\r\n").unwrap();
}

fn print_hex_arr_rev<S>(tag: &str, serial: &mut S, arr: &[u8])
where
    S: uWrite,
    <S as uWrite>::Error: Debug,
{
    ufmt::uwrite!(serial, "{} = ", tag).unwrap();
    for e in arr.iter().rev() {
        ufmt::uwrite!(serial, "{:02x}", *e).unwrap();
    }
    ufmt::uwrite!(serial, "\r\n").unwrap();
}

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);
    let mut serial = arduino_hal::default_serial!(dp, pins, 57600);

    /*
    // Uncommenting this part changes the output of hmacs
    use noble_secp256k1::awint::{cc, inlawi, inlawi_ty, Bits, InlAwi};
    use noble_secp256k1::{BigNum, Curve};

    let mut private_key = inlawi!(0x02_u512);
    let curve = Curve::secp256k1();
    let public_key = curve.multiply_simple(&mut private_key);

    let mut buf = [0; 32];
    public_key.x.to_u8_slice(&mut buf);
    print_hex_arr_rev("x", &mut serial, &buf);
    public_key.y.to_u8_slice(&mut buf);
    print_hex_arr_rev("y", &mut serial, &buf);
    */

    let h = hmac_sha256::HMAC::mac(b"hello", b"key");
    print_hex_arr(" mac", &mut serial, &h);
    let h = hmac_sha256::Hash::hash(b"hello");
    print_hex_arr("hash", &mut serial, &h);

    loop {}
}
