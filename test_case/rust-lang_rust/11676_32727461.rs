 rust
#[no_mangle]
pub extern "C" fn sk_message_serialized_size_info<M: proto::Serializable>(obj: &M) -> *proto::SerializedSizeInfo {
  let ret = proto::compute_serialized_size_of(obj);
  unsafe { cast::transmute(ret) }
}
