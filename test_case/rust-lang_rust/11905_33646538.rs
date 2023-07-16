
// Yay!
lock.lock();
do_something_under_lock();
scope!(lock.unlock())
