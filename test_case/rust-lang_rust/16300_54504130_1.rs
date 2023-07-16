 diff
 struct hoedown_renderer {
     opaque: *mut hoedown_html_renderer_state,
     blockcode: Option<extern "C" fn(*mut hoedown_buffer, *const hoedown_buffer,
                                     *const hoedown_buffer, *mut libc::c_void)>,
     blockquote: Option<extern "C" fn(*mut hoedown_buffer, *const hoedown_buffer,
                                      *mut libc::c_void)>,
     blockhtml: Option<extern "C" fn(*mut hoedown_buffer, *const hoedown_buffer,
                                     *mut libc::c_void)>,
     header: Option<extern "C" fn(*mut hoedown_buffer, *const hoedown_buffer,
                                  libc::c_int, *mut libc::c_void)>,
+    math: Option<extern "C" fn(*mut hoedown_buffer, *const hoedown_buffer,
+                                   libc::c_int, *mut libc::c_void)>,
     other: [libc::size_t, ..28],
 }
