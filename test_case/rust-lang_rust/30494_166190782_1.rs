 rust
extern {
  fn v8_function_callback_info_get_return_value(this: &mut FunctionCallbackInfo) -> &mut ReturnValue;
}

// Doesn't really matter what's inside, can still pretend there's a pointer there.
#[repr(C)]
pub struct FunctionCallbackInfo(*mut u8);
#[repr(C)]
pub struct ReturnValue(*mut u8);

impl FunctionCallbackInfo {
  pub fn GetReturnValue(&mut self) -> &mut ReturnValue {
    unsafe { v8_function_callback_info_get_return_value(self) }
  }
}

impl StringBytes {
  extern fn New(arguments: &mut v8::FunctionCallbackInfo) {
    arguments.GetReturnValue().SetWithBool(true);
  }
}
