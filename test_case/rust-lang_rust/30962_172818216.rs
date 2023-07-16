 cpp
#include<atomic>

void foo(std::atomic<long>& x) {
    x.fetch_add(2, std::memory_order_seq_cst);
    x.fetch_add(2, std::memory_order_seq_cst);
}
