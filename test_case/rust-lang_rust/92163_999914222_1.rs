rust
pub unsafe fn init() {
	let (_engine, _engine_path) = open_library_srv!("engine").expect("Failed to open engine shared library!");

	extern "C" fn __process_voice_data(client: *mut std::ffi::c_void, len: i32, data: *mut std::os::raw::c_char, xuid: i64) {
		unsafe {
			if crate::record::SV_BroadcastVoiceData(client, len, data, xuid) {
				BROADCAST_VOICE_DATA_DETOUR.get().call(client, len, data, xuid);
			}
		}
	}
