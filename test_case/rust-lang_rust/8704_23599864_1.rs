 C++
class boxed_region {
private:
    bool poison_on_free;
    memory_region *backing_region;
    rust_opaque_box *live_allocs;
// some code here ommitted
}
