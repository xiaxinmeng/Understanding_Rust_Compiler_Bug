
01:47:11] error: the type of this value must be known in this context
[01:47:11]    --> src\libstd\sys\windows\os.rs:330:65
[01:47:11]     |
[01:47:11] 330 |         assert!(!Error::from_raw_os_error(STATUS_UNSUCCESSFUL | c::FACILITY_NT_BIT as _)
[01:47:11]     |                                                                 ^^^^^^^^^^^^^^^^^^^^^^^
[01:47:11] 
[01:47:11] error: aborting due to previous error
