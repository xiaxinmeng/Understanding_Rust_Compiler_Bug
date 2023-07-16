 diff
diff --git a/src/libstd/linux_os.rs b/src/libstd/linux_os.rs
index 82b1197..d7a71d0 100644
--- a/src/libstd/linux_os.rs
+++ b/src/libstd/linux_os.rs
@@ -57,6 +57,7 @@ native mod libc {
     fn pipe(buf: *mutable fd_t) -> c_int;
     fn waitpid(pid: pid_t, &status: c_int, options: c_int) -> pid_t;
     fn readlink(path: str::sbuf, buf: str::sbuf, bufsize: size_t) -> ssize_t;
+    fn perror(msg: str::sbuf);
     fn mkdir(path: str::sbuf, mode: c_int) -> c_int;
     fn rmdir(path: str::sbuf) -> c_int;
     fn chdir(path: str::sbuf) -> c_int;
@@ -132,6 +133,7 @@ fn get_exe_path() -> option::t<fs::path> {
             if libc::readlink(proc_self_buf, path_buf, bufsize) != -1 {
                 option::some(fs::dirname(path) + fs::path_sep())
             } else {
+                libc::perror(proc_path_buf);
                 option::none
             }
         })
