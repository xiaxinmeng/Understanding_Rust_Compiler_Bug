c++
_Alignas(std::atomic_ref<int>::required_alignment) int atomic;
std::atomic_ref ref{atomic};
ref.store(1,std::memory_order::relaxed);
ref->~std::atomic_ref(); // ends the lifetime of ref
atomic = 2;
::new(&ref) std::atomic_ref{atomic}; // creates a new one in-place. This is basically equivalent to dropping the first reference, and replacing it with a new one
int val = ref->fetch_add(1, std::memory_order::relaxed);  // val is 1, since [atomics.order] requires that a read-modify-write operation reads the last value written in modification order before the corresponding write.
