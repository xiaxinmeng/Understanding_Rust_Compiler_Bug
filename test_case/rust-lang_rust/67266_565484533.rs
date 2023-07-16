rust
#[cfg(windows)]
pub fn now() -> Instant {
	use std::sync::atomic::{AtomicUsize, Ordering};
	use winapi::shared::minwindef::{FILETIME, LPFILETIME};
	use winapi::um::libloaderapi;
	use winapi::um::sysinfoapi::GetSystemTimeAsFileTime;
	static PTR: AtomicUsize = AtomicUsize::new(0);
	// FIXME: not even sure if this atomic ordering stuff is needed.
	// code adapted from https://github.com/rust-lang/rust/blob/master/src/libstd/sys/windows/compat.rs
	let addr = match PTR.load(Ordering::SeqCst) {
		0 => {
			let module = b"kernel32\0".as_ptr() as *const i8;
			let symbol = b"GetSystemTimePreciseAsFileTime\0".as_ptr() as *const i8;
			let addr = unsafe {
				let handle = libloaderapi::GetModuleHandleA(module);
				match libloaderapi::GetProcAddress(handle, symbol) as usize {
					0 => GetSystemTimeAsFileTime as usize, // fallback function
					addr => addr,
				}
			};
			PTR.store(addr, Ordering::SeqCst);
			addr
		}
		addr => addr,
	};
	let ticks = unsafe {
		let mut ft: FILETIME = std::mem::zeroed();
		type PFNGSTAFT = unsafe extern "system" fn(LPFILETIME);
		std::mem::transmute::<usize, PFNGSTAFT>(addr)(&mut ft as LPFILETIME);
		std::mem::transmute::<FILETIME, u64>(ft) - 116444736000000000
	};
	Instant::from_ticks(ticks)
}
