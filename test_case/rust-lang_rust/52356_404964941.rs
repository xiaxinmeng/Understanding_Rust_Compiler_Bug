rust
extern crate rand;
extern crate symphony_fpga;

use rand::{thread_rng, Rng};
use symphony_fpga::*;

fn rand_string(chars: &mut [u8]) {
    let charset: Vec<u8> = String::from("abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRTSUVWXYZ1234567890").into_bytes();
    let rnos: Vec<usize> = (0..chars.len())
        .map(|_| thread_rng().gen_range::<usize>(0, charset.len()))
        .collect();
    for i in 0..chars.len() {
        chars[i] = charset[rnos[i]];
    }
    chars[chars.len()-1] = '\0' as u8;
}

fn main() {
    // Initialize AWS libraries
    assert!(init_aws().is_ok());

    // Configuration
    let slot_id = 0;
    let queue_id = 0;
    let mut dma = DMAQueue::new(slot_id, queue_id).unwrap();

    // Input buffer
    let len = 1 << 10;
    let mut buf = vec![0u8; len];
    rand_string(&mut buf[..]);

    // Write data
    for channel in 0..4 {
        let write_address: i64 = 0x10000000 + channel*(1 << 34);
        assert!(dma.sync_write(&buf[..], write_address).is_ok());
    }

    // Read data
    let out = vec![0u8; len];
    for channel in 0..4 {
        let read_address:i64 = 0x10000000 + channel*(1 << 34);
        assert!(dma.sync_read(&mut buf[..], read_address).is_ok());
        assert_eq!(out, buf)
    }
}

