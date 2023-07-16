rust
pub fn main() {
    #[global_allocator]
    static ALLOCATOR: ::TAlloc = ::TAlloc;
}
