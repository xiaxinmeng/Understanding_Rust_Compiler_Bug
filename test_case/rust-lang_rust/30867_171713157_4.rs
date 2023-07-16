
F: Sized
for<'a> F: Fn<(&'a i32,)>
for<'a> &'a i32: Sized
for<'a> <F as Fn<(&'a i32,)>>::Output == $U
$U: Sized
