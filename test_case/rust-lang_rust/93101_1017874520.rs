rust
std::panic::Hook::default()
    .set_capture_preference(my_runtime_capture_preference)
    .install();
