 rust
do spawn(0) {
    // smallest possible stack size, the current default
}

do spawn(std::os::stack_size()) {
    // get the platform stack size from pthread_attr_getstacksize
}

do spawn(4 * 1024) {
    // reserve close to the known required space, to get the best of both worlds
}
