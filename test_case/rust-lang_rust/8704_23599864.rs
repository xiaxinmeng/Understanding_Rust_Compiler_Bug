 rust
struct Env { priv opaque: () }

struct BoxedRegion {
    env: *Env,
    backing_region: *MemoryRegion,
    live_allocs: *raw::Box<()>,
}
