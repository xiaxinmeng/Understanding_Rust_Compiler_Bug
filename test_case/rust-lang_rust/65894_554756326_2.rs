compile_fail
2019-11-17T15:22:00.9504235Z     |  _________^
2019-11-17T15:22:00.9504902Z 302 | | //!     fn exploit_ref_cell<T>(rc: Pin<&mut RefCell<T>>) {
2019-11-17T15:22:00.9505654Z 303 | | //!         { let p = rc.as_mut().get_pin_mut(); } // Here we get pinned access to the `T`.
2019-11-17T15:22:00.9506385Z 304 | | //!         let rc_shr: &RefCell<T> = rc.into_ref().get_ref();
2019-11-17T15:22:00.9507526Z 307 | | //!     }
2019-11-17T15:22:00.9509140Z 308 | | //!     