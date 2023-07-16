
<anon>:4:13: 4:14 error: incompatible bounds on type parameter `T`, bound `serialize::json::ToJson` does not allow unsized type [E0129]
<anon>:4 fn f<Sized? T: ToJson>(x: &T) {
                     ^
error: aborting due to previous error
