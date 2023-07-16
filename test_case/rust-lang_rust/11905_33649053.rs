 rust
lock.lock();
finally!(lock.unlock())

do_something_under_lock();
