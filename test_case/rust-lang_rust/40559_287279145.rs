
pub union ManuallyDrop<T>{ value: T, dropped: () }
    pub unsafe fn drop(slot: &mut ManuallyDrop<T>) {
        ptr::drop_in_place(&mut slot.value)
        *slot = ManuallyDrop { dropped: () };
    }
