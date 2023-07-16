rust
use std::panic::RefUnwindSafe;
use std::sync::Mutex as StdMutex;
use parking_lot::Mutex as ParkingLotMutex;

fn assert_ref_unwind_safe<T: RefUnwindSafe>(t: T) {

}

fn main() {
    assert_ref_unwind_safe(StdMutex::new(0));

    assert_ref_unwind_safe(ParkingLotMutex::new(0));
}
