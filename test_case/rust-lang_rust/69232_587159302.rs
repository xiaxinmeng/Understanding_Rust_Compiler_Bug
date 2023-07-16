
error[E0617]: can't pass `{individual function type for libc::unix::fread}` to variadic function
   --> hg_connect.rs:844:62
    |
844 |                 curl_easy_setopt(curl, CURLOPT_READFUNCTION, libc::fread);
    |                                                              ^^^^^^^^^^^
    |
help: cast the value to a function pointer:
    |
844 |                 curl_easy_setopt(curl, CURLOPT_READFUNCTION, libc::fread as unsafe extern "C" fn(*mut core::ffi::c_void, usize, usize, *mut libc::unix::FILE) -> usize);
    |                                                              ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

