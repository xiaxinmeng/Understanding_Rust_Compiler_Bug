rust
fn calculate_things(buffer: &mut [u32]) {
    let buffer: &[AtomicU32] = AtomicU32::from_mut_slice(buffer);
    crossbeam_utils::thread::scope(|s| {
        for i in 0..n_threads {
            s.spawn(|_| {
                // do work and add stuff to `buffer`.
            });
        }
    }).unwrap();
}
