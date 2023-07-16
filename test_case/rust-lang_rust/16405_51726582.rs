 rust
extern crate sync;
use sync::mutex::Mutex;

struct Node {
   mutex : Mutex
}
struct Link {
   to : &Node,
   from : &Node,
}

fn deadlock(link : &mut Link) {
  let to_guard = link.to.mutex.lock();
  let from_guard = link.from.mutex.lock();   // Deadlock can occur here
  drop(to_guard);
  drop(from_guard);
}
