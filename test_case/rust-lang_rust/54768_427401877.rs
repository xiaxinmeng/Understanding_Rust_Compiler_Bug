
T: Sync <=> &T: Send
T: RefUnwindSafe <=> &T: UnwindSafe
T: Sync => T: RefUnwindSafe
&T: Send => T: RefUnwindSafe
T: Sync => &T: UnwindSafe
&T: Send => &T: UnwindSafe
