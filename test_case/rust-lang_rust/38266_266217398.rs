
#[link(name = "libsoundio")]
#[link(name = "ole32")]
extern {
	pub fn soundio_version_string() -> *const c_char;
        ....
