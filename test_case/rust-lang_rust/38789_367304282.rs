rust
#[inline]
fn baz() { }

#[target_device(host, nvptx(sm = "40"), spirv(version = "1.0"))]
fn bar(...) { ... }

#[target_device_kernel(host, nvptx(sm = "40"), spirv(version = "1.0"))]
fn foo(...) { 
  #[device] bar(...); // device attribute indicates that this fn is a device fn
  baz(); // this function will be used as is
}
