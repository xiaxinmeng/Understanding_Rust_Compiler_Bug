 rust
static CELL: AtomicCell<Cell<u32>> = AtomicCell::new();
static CELL: AtomicCell<Cell<Sender<u32>>> = AtomicCell::new();
static CELL: AtomicCell<Arc<Cell<u32>>> = AtomicCell::new();
static CELL: AtomicCell<Arc<Cell<Sender<u32>>>> = AtomicCell::new();
static CELL: AtomicCell<Arc<RefCell<u32>>> = AtomicCell::new();
static CELL: AtomicCell<Arc<RefCell<Sender<u32>>>> = AtomicCell::new();
