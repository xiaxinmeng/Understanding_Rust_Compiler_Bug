 rust
(self.into():PathBuf).do_something()
self.into():PathBuf.do_something()
self.into::<PathBuf>().do_something()
self.into<PathBuf>().do_something() // what we wanted, but can't get yet
self.into_path_buf().do_something()
self.into(PathBuf).do_something() // magical
