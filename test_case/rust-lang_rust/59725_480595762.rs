rust
// Leaving out `wake_by_ref` here... it would ofc be included in a real implementation.

static EMBRIO_WAKER_RAW_WAKER_VTABLE: RawWakerVTable = RawWakerVTable {
    clone: |data| unsafe { (*(data as *const EmbrioWaker)).raw_waker() },
    wake: |data| unsafe { (*(data as *const EmbrioWaker)).wake() } },
    drop,
};

    pub(crate) fn raw_waker(&'static self) -> RawWaker {
        RawWaker::new(
            self as *const _ as *const (),
            &EMBRIO_WAKER_RAW_WAKER_VTABLE,
        )
    }
