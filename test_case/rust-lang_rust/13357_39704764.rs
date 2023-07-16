 rust
use sync::mutex::{MUTEX_INIT, StaticMutex};

fn init(x: u32) -> i32 {
    static mut init_mutex: StaticMutex = MUTEX_INIT;
    static mut inited: bool = false;

    unsafe {
        let _guard = init_mutex.lock();
        if inited { return 0 }

        ret = my_lib_init(x);
        inited = ret >= 0;
        ret
    }
}
