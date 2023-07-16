
trait ObjectAllocator<T> {
    fn alloc() -> T;
    fn free(t T);
}
