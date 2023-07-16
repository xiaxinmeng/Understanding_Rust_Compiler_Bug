rust
macro_rules! volatile_read_array {
    ($src:expr, $dst:expr, $ty:ty, $count:expr) => {
        volatile_read_array!(
            $src, $dst, $ty, $count, 1024, 512, 256, 128, 64, 32, 16, 8, 4, 2, 1
        )
    };
    ($src:expr, $dst:expr, $ty:ty, $count:expr, $($N:expr),* $(,)?) => {
        match $count {
            $(
                c if c >= $N => {
                    ptr::write($dst as *mut [$ty; $N], ptr::read_volatile($src as *const [$ty; $N]));
                    $N
                }
            )*
            _ => unreachable!("count is 0"),
        }
    };
}

unsafe fn volatile_read_memory<T>(mut src: *const T, mut dst: *mut T, mut count: usize) {
    while count > 0 {
        let copied = volatile_read_array!(src, dst, T, count);
        src = src.add(copied);
        dst = dst.add(copied);
        count -= copied;
    }
}

macro_rules! volatile_write_array {
    ($src:expr, $dst:expr, $ty:ty, $count:expr) => {
        volatile_write_array!(
            $src, $dst, $ty, $count, 1024, 512, 256, 128, 64, 32, 16, 8, 4,  2, 1
        )
    };
    ($src:expr, $dst:expr, $ty:ty, $count:expr, $($N:expr),* $(,)?) => {
        match $count {
            $(
                c if c >= $N => {
                    write_volatile($dst as *mut [$ty; $N], ptr::read($src as *const [$ty; $N]));
                    $N
                }
            )*
            _ => unreachable!("count is 0"),
        }
    };
}

unsafe fn volatile_write_memory<T>(mut src: *const T, mut dst: *mut T, mut count: usize) {
    while count > 0 {
        let copied = volatile_write_array!(src, dst, T, count);
        src = src.add(copied);
        dst = dst.add(copied);
        count -= copied;
    }
}
