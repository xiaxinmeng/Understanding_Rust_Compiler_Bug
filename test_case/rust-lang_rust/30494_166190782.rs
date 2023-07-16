 rust
#[repr(C)]
pub struct FunctionCallbackInfo(*mut *mut FunctionCallbackInfo);

...

impl StringBytes {
  extern fn New(arguments: v8::FunctionCallbackInfo) {
    arguments.GetReturnValue().SetWithBool(true);
  }
}
