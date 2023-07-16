
(&*(&MaybeUninit::new(arr) as *const _ as *const MaybeUninit<_>)).assume_init_read()
