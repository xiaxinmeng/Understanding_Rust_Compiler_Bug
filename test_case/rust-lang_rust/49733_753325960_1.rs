
#![feature(maybe_uninit_extra, new_uninit)]

macro_rules! boxed {
    ($e:expr) => {
        {
            let mut ws = Box::new_uninit();
            ws.write($e);
            unsafe { ws.assume_init() }
        }
    }
}
