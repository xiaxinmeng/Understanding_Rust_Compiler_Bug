rust
[1,2,3].split_array_ref<4>().is_none(); /// Would make sense if the split fails

/// It's useful for network code, to check if prefix has length at least 4 to decode the message.
/// This code can avoid any unwraps, because `u32::from_le_bytes` need array of 4 elements as input. 
if let Some((left, _) = [1,2,3,4,5].split_array_ref<4>() {
   println!("{}", u32::from_le_bytes(left));
}
