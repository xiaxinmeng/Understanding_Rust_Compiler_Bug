rust
#[thread_local]
pub static FOO: once_cell::unsync::Lazy<RefCell<u32>> = Lazy::new(|| RefCell::new(1));
