
macro_rules! boxed {
    ($e:expr) => {
        {
            let mut ws = Box::<typeof($e)>::new_uninit();
            let contents = $e;
            unsafe {
                // type inference is forbidden for raw ptrs, hence the need for typeof
                ws.as_mut_ptr().write(contents);
                ws.assume_init()
            }
        }
    }
}
