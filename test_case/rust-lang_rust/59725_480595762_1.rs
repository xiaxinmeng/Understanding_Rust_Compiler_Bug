rust
static EMBRIO_WAKER_RAW_WAKER_VTABLE: RawWakerVTable<EmbrioWaker> = RawWakerVTable {
    clone: |data| unsafe { (*data).raw_waker() },
    wake: |data| unsafe { (*data).wake() } },
    drop,
};

    pub(crate) fn raw_waker(&'static self) -> RawWaker {
        RawWaker::new(
            self as *const _,
            &EMBRIO_WAKER_RAW_WAKER_VTABLE,
        )
    }
