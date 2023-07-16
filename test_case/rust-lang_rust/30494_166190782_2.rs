 cpp
ReturnValue<Value> v8_function_callback_info_get_return_value(FunctionCallbackInfo<Value> *callbackInfo) {
  return callbackInfo->GetReturnValue();
}
