rust
let mut ar_session: *mut ArSession = ::std::ptr::null_mut();
ArSession_create(env, application_context, &mut ar_session);
