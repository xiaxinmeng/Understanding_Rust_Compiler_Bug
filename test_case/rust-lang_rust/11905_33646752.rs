
scope!(lock.unlock());
// what if we fail here? the lock isn't locked
lock.lock();
