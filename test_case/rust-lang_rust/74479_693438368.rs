rust
if unsafe { libc::sched_getaffinity(0, mem::size_of::<libc::cpu_set_t>(), &mut set) } == 0 {
    let mut count: u32 = 0;
    for i in 0..libc::CPU_SETSIZE as usize {
        if unsafe { libc::CPU_ISSET(i, &set) } {
            count += 1
        }
    }
    count as usize
} else {
    // `libc::_SC_NPROCESSORS_ONLN` code we're currently using
}
