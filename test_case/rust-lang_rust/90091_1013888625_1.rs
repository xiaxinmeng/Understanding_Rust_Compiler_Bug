rust
let ary = [127,0,0,1,55,66];

if ary.len() >= 4 {
   let (ip,_) = ary.split_at(4);
   ///. ...
}
