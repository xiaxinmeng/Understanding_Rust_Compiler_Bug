rust
// [dependencies]
// crossbeam-utils = "=0.8.8"

use crossbeam_utils::atomic::AtomicCell;
use std::num::NonZeroU128;
use std::thread;

enum Enum {
    NeverConstructed,
    Cell(AtomicCell<NonZeroU128>),
}

static STATIC: Enum = Enum::Cell(AtomicCell::new(match NonZeroU128::new(1) {
    Some(nonzero) => nonzero,
    None => unreachable!(),
}));

fn main() {
    thread::spawn(|| {
        let cell = match &STATIC {
            Enum::NeverConstructed => unreachable!(),
            Enum::Cell(cell) => cell,
        };
        let x = NonZeroU128::new(0xFFFFFFFF_FFFFFFFF_00000000_00000000).unwrap();
        let y = NonZeroU128::new(0x00000000_00000000_FFFFFFFF_FFFFFFFF).unwrap();
        loop {
            cell.store(x);
            cell.store(y);
        }
    });

    loop {
        if let Enum::NeverConstructed = STATIC {
            unreachable!(":(");
        }
    }
}
