rust
let ary = [127,0,0,1,55,66];

if let Some((ip,_)) = ary.try_split_at(4) {
  /// ...
}
