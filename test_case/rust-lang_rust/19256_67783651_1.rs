
<anon>:5:15: 5:16 error: incompatible bounds on type parameter `T`, bound `MaybeSized` does not allow unsized type [E0129]
<anon>:5 fn foo<Sized? T: MaybeSized>(f: &T){}
                       ^
error: aborting due to previous error
