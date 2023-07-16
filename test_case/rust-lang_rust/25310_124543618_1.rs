
 #[stable(feature = "env", since = "1.0.0")]
-pub fn var_os<K: ?Sized>(key: &K) -> Option<OsString> where K: AsRef<OsStr> {
+pub fn var_os<K: AsRef<OsStr>>(key: K) -> Option<OsString> {
     let _g = ENV_LOCK.lock();
     os_imp::getenv(key.as_ref())
 }
