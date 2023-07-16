diff
#[repr(C)]
#[derive(Copy, Clone)]
pub struct napi_env__ {
  _unused: [u8; 0],
}

/// Env ptr
pub type napi_env = *mut napi_env__;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct napi_value__ {
  _unused: [u8; 0],
}

/// JsValue ptr
pub type napi_value = *mut napi_value__;


#[derive(Clone, Copy)]
pub struct Value {
  pub env: napi_env,
  pub value: napi_value,
  pub value_type: ValueType,
}

pub struct JsObject(pub(crate) Value);

impl JsObject {
  #[inline]
  pub fn create_named_method(&mut self, name: &str, function: Callback) -> Result<()> {
    let mut js_function = ptr::null_mut();
    let len = name.len();
    let name = CString::new(name.as_bytes())?;
+   println!("{:p}", self.0.env);
    check_status!(unsafe {
      sys::napi_create_function(
        self.0.env,
        name.as_ptr(),
        len,
        Some(function),
        ptr::null_mut(),
        &mut js_function,
      )
    })?;
+   println!("{:p}", self.0.env);
    check_status!(unsafe {
      sys::napi_set_named_property(self.0.env, self.0.value, name.as_ptr(), js_function)
    })
  }
}
