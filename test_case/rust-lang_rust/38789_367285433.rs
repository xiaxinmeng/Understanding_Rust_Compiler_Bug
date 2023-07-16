rust
#[target_feature(enabled = "ptx_device_fn")] unsafe fn a_device_fn(...) { ... }
#[target_feature(enabled = "ptx_kernel")] unsafe fn a_kernel(...) { ... a_device_fn(...) ... }

unsafe fn bar() {
   cuda::driver::launch(x, y, z, w, a_kernel);
   cuda::driver::launch(x, y, z, w, #[target_feature(enabled = "ptx_kernel")] |...| a_kernel(...) );
}
