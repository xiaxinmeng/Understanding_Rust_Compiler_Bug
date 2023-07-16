 diff
 struct MyOpaque {
     dfltblk: extern "C" fn(*mut hoedown_buffer, *const hoedown_buffer,
                            *const hoedown_buffer, *mut libc::c_void),
     toc_builder: Option<TocBuilder>,
+    has_math: bool,
 }
