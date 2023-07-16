 Rust
extern "C" fn read_callback(dsp_state: *mut ffi::FMOD_DSP_STATE, in_buffer: *mut c_float, out_buffer: *mut c_float, length: c_uint, in_channels: c_int, out_channels: c_int) -> fmod::Result {
     unsafe {
         let mut v_in_buffer = Vec::from_slice(std::c_vec::CVec::new(in_buffer, length as uint * in_channels as uint + out_channels as uint).as_slice());
        ...
}

let mut ptr = Vec::from_elem(spectrum_size, 0f32);
ffi::FMOD_System_GetSpectrum(self.system, ptr.as_mut_ptr(), spectrum_size as c_int, channel_offset, window_type)
