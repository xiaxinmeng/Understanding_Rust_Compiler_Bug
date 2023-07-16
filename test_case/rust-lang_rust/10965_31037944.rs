 rust
extern mod native;
extern mod green;

use std::task::TaskOpts;

#[start]
fn start(argc: int, argv: **u8) -> int {
    do native::start(argc, argv) {
        let mut pool = green::SchedPool::new(green::PoolConfig::new());
        let (p, c) = Chan::new();
        do pool.spawn(TaskOpts::new()) {
            println!("green pool");
            c.send(());
        }
        p.recv();
        pool.shutdown();
    }
}
