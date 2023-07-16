
kurtis:rust $ git grep epoch; git grep Epoch; git grep EPOCH
src/Cargo.lock: "crossbeam-epoch 0.3.0 (registry+https://github.com/rust-lang/crates.io-index)",
src/Cargo.lock:name = "crossbeam-epoch"
src/Cargo.lock:"checksum crossbeam-epoch 0.3.0 (registry+https://github.com/rust-lang/crates.io-index)" = "59796cc6cbbdc6bb319161349db0c3250ec73ec7fcb763a51065ec4e2e158552"
src/ci/docker/scripts/android-start-emulator.sh:# Using the default qemu2 engine makes time::tests::since_epoch fails because
src/ci/docker/scripts/android-start-emulator.sh:# the emulator date is set to unix epoch (in armeabi-v7a-18 image). Using
src/librustc_incremental/persist/fs.rs:    let micros_since_unix_epoch = u64::from_str_radix(s, INT_ENCODE_BASE as u32);
src/librustc_incremental/persist/fs.rs:    if micros_since_unix_epoch.is_err() {
src/librustc_incremental/persist/fs.rs:    let micros_since_unix_epoch = micros_since_unix_epoch.unwrap();
src/librustc_incremental/persist/fs.rs:    let duration = Duration::new(micros_since_unix_epoch / 1_000_000,
src/librustc_incremental/persist/fs.rs:                                 1000 * (micros_since_unix_epoch % 1_000_000) as u32);
src/libstd/sys/cloudabi/abi/cloudabi.rs:  /// The epoch of this clock is undefined. The absolute
src/libstd/time.rs:        let one_second_from_epoch = UNIX_EPOCH + Duration::new(1, 0);
src/libstd/time.rs:        let one_second_from_epoch2 = UNIX_EPOCH + Duration::new(0, 500_000_000)
src/libstd/time.rs:        assert_eq!(one_second_from_epoch, one_second_from_epoch2);
src/libstd/time.rs:    fn since_epoch() {
src/test/run-pass/issue-29540.rs:    pub mon_min_osdmap_epochs: String,
src/test/run-pass/issue-29540.rs:    pub mon_max_pgmap_epochs: String,
src/test/run-pass/issue-29540.rs:    pub mon_max_log_epochs: String,
src/test/run-pass/issue-29540.rs:    pub mon_max_mdsmap_epochs: String,
src/test/run-pass/issue-29540.rs:    pub osd_map_share_max_epochs: String,
src/test/run-pass/issue-29540.rs:    pub osd_pg_epoch_persisted_max_stale: String,
RELEASES.md:  * [`UNIX_EPOCH`]
RELEASES.md:[`UNIX_EPOCH`]: http://doc.rust-lang.org/nightly/std/time/constant.UNIX_EPOCH.html
src/bootstrap/util.rs:        let start = self.start_time.duration_since(UNIX_EPOCH);
src/bootstrap/util.rs:        let finish = end_time.duration_since(UNIX_EPOCH);
src/librustc/dep_graph/graph.rs:        use std::time::{SystemTime, UNIX_EPOCH};
src/librustc/dep_graph/graph.rs:        let duration = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
src/librustc_incremental/persist/fs.rs:use std::time::{UNIX_EPOCH, SystemTime, Duration};
src/librustc_incremental/persist/fs.rs:    let mut best_candidate = (UNIX_EPOCH, None);
src/librustc_incremental/persist/fs.rs:    let duration = timestamp.duration_since(UNIX_EPOCH).unwrap();
src/librustc_incremental/persist/fs.rs:    Ok(UNIX_EPOCH + duration)
src/librustc_incremental/persist/fs.rs:            (UNIX_EPOCH + Duration::new(4, 0), PathBuf::from("4"), None),
src/librustc_incremental/persist/fs.rs:            (UNIX_EPOCH + Duration::new(1, 0), PathBuf::from("1"), None),
src/librustc_incremental/persist/fs.rs:            (UNIX_EPOCH + Duration::new(5, 0), PathBuf::from("5"), None),
src/librustc_incremental/persist/fs.rs:            (UNIX_EPOCH + Duration::new(3, 0), PathBuf::from("3"), None),
src/librustc_incremental/persist/fs.rs:            (UNIX_EPOCH + Duration::new(2, 0), PathBuf::from("2"), None),
src/librustc_incremental/persist/fs.rs:        let time = UNIX_EPOCH + Duration::new(i * 1_434_578, (i as u32) * 239_000);
src/libstd/sys/cloudabi/time.rs:pub const UNIX_EPOCH: SystemTime = SystemTime { t: 0 };
src/libstd/sys/redox/net/mod.rs:        let time = time::SystemTime::now().duration_since(time::UNIX_EPOCH).unwrap();
src/libstd/sys/redox/time.rs:pub const UNIX_EPOCH: SystemTime = SystemTime {
src/libstd/sys/unix/time.rs:pub use self::inner::{Instant, SystemTime, UNIX_EPOCH};
src/libstd/sys/unix/time.rs:    pub const UNIX_EPOCH: SystemTime = SystemTime {
src/libstd/sys/unix/time.rs:    pub const UNIX_EPOCH: SystemTime = SystemTime {
src/libstd/sys/wasm/time.rs:pub const UNIX_EPOCH: SystemTime = SystemTime(Duration::from_secs(0));
src/libstd/sys/windows/time.rs:const INTERVALS_TO_UNIX_EPOCH: u64 = 11_644_473_600 * INTERVALS_PER_SEC;
src/libstd/sys/windows/time.rs:pub const UNIX_EPOCH: SystemTime = SystemTime {
src/libstd/sys/windows/time.rs:        dwLowDateTime: INTERVALS_TO_UNIX_EPOCH as u32,
src/libstd/sys/windows/time.rs:        dwHighDateTime: (INTERVALS_TO_UNIX_EPOCH >> 32) as u32,
src/libstd/time.rs:/// Although a `SystemTime` cannot be directly inspected, the [`UNIX_EPOCH`]
src/libstd/time.rs:/// [`UNIX_EPOCH`]: ../../std/time/constant.UNIX_EPOCH.html
src/libstd/time.rs:/// measurement lies, and using `UNIX_EPOCH + duration` can be used to create a
src/libstd/time.rs:/// use std::time::{SystemTime, UNIX_EPOCH};
src/libstd/time.rs:/// match SystemTime::now().duration_since(UNIX_EPOCH) {
src/libstd/time.rs:///     Err(_) => panic!("SystemTime before UNIX EPOCH!"),
src/libstd/time.rs:pub const UNIX_EPOCH: SystemTime = SystemTime(time::UNIX_EPOCH);
src/libstd/time.rs:    use super::{Instant, SystemTime, Duration, UNIX_EPOCH};
src/libstd/time.rs:        let one_second_from_epoch = UNIX_EPOCH + Duration::new(1, 0);
src/libstd/time.rs:        let one_second_from_epoch2 = UNIX_EPOCH + Duration::new(0, 500_000_000)
src/libstd/time.rs:        let a = ts.duration_since(UNIX_EPOCH).unwrap();
src/libstd/time.rs:        let b = ts.duration_since(UNIX_EPOCH - Duration::new(1, 0)).unwrap();
