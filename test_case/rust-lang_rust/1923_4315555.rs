
// Trying to generate races between killing tasks, sending, and deleting ports
fn main() {
    for iter::repeat(100u) {
        let builder = task::builder();
        task::unsupervise(builder);
        do task::run(builder) {
            let po = comm::port();
            let ch = comm::chan(po);
            let builder = task::builder();
            task::unsupervise(builder);
            do task::run(builder) {
                comm::send(ch, ());
            }
            do task::spawn {
                fail;
            }
        }
    }
}
