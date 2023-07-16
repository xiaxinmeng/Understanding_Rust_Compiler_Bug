rust
let ary = [127,0,0,1,55,66];

if let Some((ip,_)) = ary.try_split_array_ref<4>() {
  /// This makes ip a reference to 4 bytes array.
  /// ...
}
