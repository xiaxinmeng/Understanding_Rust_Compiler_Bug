rust
struct MyGlobalStatefulAlloc;

static mut MyGlobalAllocState: StatefulAlloc = StatefulAlloc::default();

impl Alloc for MyGlobalStatefulAlloc { /* impl using MyGlobalAllocState */ }
