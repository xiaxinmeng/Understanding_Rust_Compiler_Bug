 rust
extern mod native;

static mut set: bool = false;
#[start]
fn start(argc: int, argv: **u8) -> int {
    native::start(argc, argv, proc() spawn(proc() unsafe {set = true}));

    if unsafe {set} {0} else {1}
}
