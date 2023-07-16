
use std::thread;
use std::sync::{Arc, Mutex};

static NUM_THREADS: u8 = 4;
static NUM_REP: u32 = 100000;
pub static mut PROG_SUM: u8 = 0;

fn unsafe_sum() {
    let mut thread_handles = vec![];

    for _ in 0..NUM_THREADS {
        thread_handles.push(thread::spawn(move || {
            for i in 1..11 {
                unsafe {
                    PROG_SUM += i;
                }
            }
        }));
    }

    for handle in thread_handles {
        let _ = handle.join();
    }
    let fin_prog_sum;
    unsafe {
        fin_prog_sum = PROG_SUM;
    }
    assert_eq! (NUM_THREADS * 55, fin_prog_sum);
    unsafe {
        PROG_SUM = 0;
    }
}


fn safe_sum() {
    let sum = Arc::new(Mutex::new(0u8));
    let mut thread_handles = vec![];
    for _ in 0..NUM_THREADS {
        let sum_thread_handle = sum.clone();
        thread_handles.push(thread::spawn(move || {
            for i in 1..11 {
                let mut data = sum_thread_handle.lock().unwrap();
                *data += i;
            }
        }));
    }
    for handle in thread_handles {
        let _ = handle.join();
    }
    let final_sum = *sum.lock().unwrap();
    assert_eq! (NUM_THREADS * 55, final_sum);
}

fn main() {
    for _ in 0..NUM_REP {
        safe_sum();
    }
    println!("safe sum done");
    for _ in 0..NUM_REP {
        unsafe_sum();
    }
    println!("unsafe sum done");
}
