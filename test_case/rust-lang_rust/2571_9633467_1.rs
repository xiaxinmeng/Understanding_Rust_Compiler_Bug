
extern "C" CDECL void
upcall_call_shim_on_c_stack(void *args, void *fn_ptr) {
    rust_task *task = rust_get_current_task();

    try {
        task->call_on_c_stack(args, fn_ptr);
    } catch (...) {
        // Logging here is not reliable
        assert(false && "Foreign code threw an exception");
    }
}
