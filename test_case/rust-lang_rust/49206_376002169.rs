rust
struct Event {
  address: usize,
  kind: EventKind
}

enum EventKind { Acquire, Release }

// A global event registry thing
static mut EVENT_LIST : Mutex<Vec<Event>> = Mutex::new(Vec::new());

fn add_event(e: Event) {
  unsafe { EVENT_LIST.lock().push(e) }
}

fn get_events() -> Vec<Event> {
  unsafe { EVENT_LIST.lock().clone() }
}

// Now comes the bad code
struct Foo(u32);

impl !Sync for Foo {}

impl Foo {
  fn acquire_release(&self) {
    add_event(Event { address: self as *const _ as usize, EventKind::Acquire });
    add_event(Event { address: self as *const _ as usize, EventKind::Release });
  }
}

const F: &'static Foo = &Foo(0);

fn main() {
  std::thread::spawn(|| F.acquire_release());
  F.acquire_release();
}
