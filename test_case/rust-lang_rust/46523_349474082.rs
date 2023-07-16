rust
// while_let_loops.rs
#[cfg(cfail1)]
pub fn change_continue_label() {
    let mut _x = 0;
    'outer: while true {
        'inner: while true {
            _x = 1;
            continue 'inner;
        }
    }
}

#[cfg(not(cfail1))]
#[rustc_clean(cfg="cfail2", except="HirBody, MirValidated")]
#[rustc_clean(cfg="cfail3")]
pub fn change_continue_label() {
    let mut _x = 0;
    'outer: while true {
        'inner: while true {
            _x = 1;
            continue 'outer;
        }
    }
}

// while_let_loops.rs
#[cfg(cfail1)]
pub fn change_continue_label() {
    let mut _x = 0;
    'outer: while let Some(0u32) = None {
        'inner: while let Some(0u32) = None {
            _x = 1;
            continue 'inner;
        }
    }
}

#[cfg(not(cfail1))]
#[rustc_clean(cfg="cfail2", except="HirBody, MirValidated, MirOptimized, TypeckTables")]
#[rustc_clean(cfg="cfail3")]
pub fn change_continue_label() {
    let mut _x = 0;
    'outer: while let Some(0u32) = None {
        'inner: while let Some(0u32) = None {
            _x = 1;
            continue 'outer;
        }
    }
}
