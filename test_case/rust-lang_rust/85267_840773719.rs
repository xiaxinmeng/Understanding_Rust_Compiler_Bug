
Breakpoint 1, gdb_pretty_test::test () at src/main.rs:5
5           let bare_vec = vec![1u8, 2, 3];
6           let newtype_vec = TrivialNewtype(vec![1u8, 2, 3]);
7           let placeholder = 12;
$1 = Vec(size=3) = {1, 2, 3}
$2 = gdb_pretty_test::TrivialNewtype (Vec(size=3) = {1, 2, 3})
