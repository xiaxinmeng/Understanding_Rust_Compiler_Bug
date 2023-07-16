text
warning: function is marked #[no_mangle], but not exported
 --> src\panic.rs:6:1
  |
6 |   fn panic(_info: &PanicInfo) -> ! {
  |   ^
  |   |
  |  _help: try making it public: `pub`
  | |
7 | |     unsafe { intrinsics::abort() }
8 | | }
  | |_^
  |
  = note: #[warn(private_no_mangle_fns)] on by default
