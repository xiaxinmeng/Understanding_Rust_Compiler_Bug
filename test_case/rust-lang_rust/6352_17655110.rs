
{ i8*, { i32, i32, [8 x i8] } } { i8* getelementptr inbounds ([5 x i8]* @const55, i32 0, i32 0), { i32, i32, [8 x i8] } { i32 0, i32 0, [8 x i8] undef } } { i8*, { i32, double, [0 x i8] } } undef
error: internal compiler error: const expr(28945: ConstantSpec{name: &NONE_name as u8 as *libc::c_char, value: IntVal(0),}) of type dom::bindings::utils::ConstantSpec has size 20 instead of 24
rust: task failed at 'explicit failure', /home/aydin.kim/servo/webconv/arm_arranged/src/rust/src/libsyntax/diagnostic.rs:92
rust: task failed at 'explicit failure', /home/aydin.kim/servo/webconv/arm_arranged/src/rust/src/librustc/rustc.rc:349
rust: domain main @0x1d20410 root task failed
make: ** [servo] Error 101
