
/*rust*/ unsafe extern "C" fn getaddrinfo(retval: *mut c_void, _mdata: *mut c_void, mut ap: ...) -> c_int
// is equivalent to 
/*c/c++*/ extern "C" int getaddrinfo(void *retval, void *mdata __unused, ...)
