
//
// (I.e. the v0 and v2 assertions pass, and the v1 and v3 Debug::fmt attempts
// are overriden by different methods in the Access trait below.)
//
// On rustc 1.51.0-nightly (22ddcd1a1 2021-01-22), this prints:
//
// 