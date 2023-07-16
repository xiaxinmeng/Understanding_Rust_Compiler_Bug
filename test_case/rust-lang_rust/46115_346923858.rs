
[00:57:37] error[E0432]: unresolved import `sys::net::Socket`
[00:57:37]   --> /checkout/src/libstd/sys/unix/ext/net.rs:39:5
[00:57:37]    |
[00:57:37] 39 | use sys::net::Socket;
[00:57:37]    |     ^^^^^^^^^^^^^^^^ no `Socket` in `sys::redox::net`
[00:57:37] 
[00:57:38] error[E0119]: conflicting implementations of trait `sys::unix_ext::io::AsRawFd` for type `process::ChildStdin`:
[00:57:38]    --> /checkout/src/libstd/sys/unix/ext/process.rs:154:1
[00:57:38]     |
[00:57:38] 154 | / impl AsRawFd for process::ChildStdin {
[00:57:38] 155 | |     fn as_raw_fd(&self) -> RawFd {
[00:57:38] 156 | |         self.as_inner().fd().raw()
[00:57:38] 157 | |     }
[00:57:38] 158 | | }
[00:57:38]     | |_^ conflicting implementation for `process::ChildStdin`
[00:57:38]     | 
[00:57:38]    ::: /checkout/src/libstd/sys/redox/ext/process.rs
[00:57:38]     |
[00:57:38] 144 | / impl AsRawFd for process::ChildStdin {
[00:57:38] 145 | |     fn as_raw_fd(&self) -> RawFd {
[00:57:38] 146 | |         self.as_inner().fd().raw()
[00:57:38] 147 | |     }
[00:57:38] 148 | | }
[00:57:38]     | |_- first implementation here
[00:57:38] 
[00:57:38] error[E0119]: conflicting implementations of trait `sys::unix_ext::io::AsRawFd` for type `process::ChildStdout`:
[00:57:38]    --> /checkout/src/libstd/sys/unix/ext/process.rs:161:1
[00:57:38]     |
[00:57:38] 161 | / impl AsRawFd for process::ChildStdout {
[00:57:38] 162 | |     fn as_raw_fd(&self) -> RawFd {
[00:57:38] 163 | |         self.as_inner().fd().raw()
[00:57:38] 164 | |     }
[00:57:38] 165 | | }
[00:57:38]     | |_^ conflicting implementation for `process::ChildStdout`
[00:57:38]     | 
[00:57:38]    ::: /checkout/src/libstd/sys/redox/ext/process.rs
[00:57:38]     |
[00:57:38] 151 | / impl AsRawFd for process::ChildStdout {
[00:57:38] 152 | |     fn as_raw_fd(&self) -> RawFd {
[00:57:38] 153 | |         self.as_inner().fd().raw()
[00:57:38] 154 | |     }
[00:57:38] 155 | | }
[00:57:38]     | |_- first implementation here
[00:57:38] 
[00:57:38] error[E0119]: conflicting implementations of trait `sys::unix_ext::io::AsRawFd` for type `process::ChildStderr`:
[00:57:38]    --> /checkout/src/libstd/sys/unix/ext/process.rs:168:1
[00:57:38]     |
[00:57:38] 168 | / impl AsRawFd for process::ChildStderr {
[00:57:38] 169 | |     fn as_raw_fd(&self) -> RawFd {
[00:57:38] 170 | |         self.as_inner().fd().raw()
[00:57:38] 171 | |     }
[00:57:38] 172 | | }
[00:57:38]     | |_^ conflicting implementation for `process::ChildStderr`
[00:57:38]     | 
[00:57:38]    ::: /checkout/src/libstd/sys/redox/ext/process.rs
[00:57:38]     |
[00:57:38] 158 | / impl AsRawFd for process::ChildStderr {
[00:57:38] 159 | |     fn as_raw_fd(&self) -> RawFd {
[00:57:38] 160 | |         self.as_inner().fd().raw()
[00:57:38] 161 | |     }
[00:57:38] 162 | | }
[00:57:38]     | |_- first implementation here
[00:57:38] 
[00:57:38] error[E0119]: conflicting implementations of trait `sys::unix_ext::io::FromRawFd` for type `process::Stdio`:
[00:57:38]    --> /checkout/src/libstd/sys/unix/ext/process.rs:145:1
[00:57:38]     |
[00:57:38] 145 | / impl FromRawFd for process::Stdio {
[00:57:38] 146 | |     unsafe fn from_raw_fd(fd: RawFd) -> process::Stdio {
[00:57:38] 147 | |         let fd = sys::fd::FileDesc::new(fd);
[00:57:38] 148 | |         let io = sys::process::Stdio::Fd(fd);
[00:57:38] 149 | |         process::Stdio::from_inner(io)
[00:57:38] 150 | |     }
[00:57:38] 151 | | }
[00:57:38]     | |_^ conflicting implementation for `process::Stdio`
[00:57:38]     | 
[00:57:38]    ::: /checkout/src/libstd/sys/redox/ext/process.rs
[00:57:38]     |
[00:57:38] 135 | / impl FromRawFd for process::Stdio {
[00:57:38] 136 | |     unsafe fn from_raw_fd(fd: RawFd) -> process::Stdio {
[00:57:38] 137 | |         let fd = sys::fd::FileDesc::new(fd);
[00:57:38] 138 | |         let io = sys::process::Stdio::Fd(fd);
[00:57:38] 139 | |         process::Stdio::from_inner(io)
[00:57:38] 140 | |     }
[00:57:38] 141 | | }
[00:57:38]     | |_- first implementation here
[00:57:38] 
[00:57:38] error[E0119]: conflicting implementations of trait `sys::unix_ext::io::IntoRawFd` for type `process::ChildStdin`:
[00:57:38]    --> /checkout/src/libstd/sys/unix/ext/process.rs:175:1
[00:57:38]     |
[00:57:38] 175 | / impl IntoRawFd for process::ChildStdin {
[00:57:38] 176 | |     fn into_raw_fd(self) -> RawFd {
[00:57:38] 177 | |         self.into_inner().into_fd().into_raw()
[00:57:38] 178 | |     }
[00:57:38] 179 | | }
[00:57:38]     | |_^ conflicting implementation for `process::ChildStdin`
[00:57:38]     | 
[00:57:38]    ::: /checkout/src/libstd/sys/redox/ext/process.rs
[00:57:38]     |
[00:57:38] 165 | / impl IntoRawFd for process::ChildStdin {
[00:57:38] 166 | |     fn into_raw_fd(self) -> RawFd {
[00:57:38] 167 | |         self.into_inner().into_fd().into_raw()
[00:57:38] 168 | |     }
[00:57:38] 169 | | }
[00:57:38]     | |_- first implementation here
[00:57:38] 
[00:57:38] error[E0119]: conflicting implementations of trait `sys::unix_ext::io::IntoRawFd` for type `process::ChildStdout`:
[00:57:38]    --> /checkout/src/libstd/sys/unix/ext/process.rs:182:1
[00:57:38]     |
[00:57:38] 182 | / impl IntoRawFd for process::ChildStdout {
[00:57:38] 183 | |     fn into_raw_fd(self) -> RawFd {
[00:57:38] 184 | |         self.into_inner().into_fd().into_raw()
[00:57:38] 185 | |     }
[00:57:38] 186 | | }
[00:57:38]     | |_^ conflicting implementation for `process::ChildStdout`
[00:57:38]     | 
[00:57:38]    ::: /checkout/src/libstd/sys/redox/ext/process.rs
[00:57:38]     |
[00:57:38] 172 | / impl IntoRawFd for process::ChildStdout {
[00:57:38] 173 | |     fn into_raw_fd(self) -> RawFd {
[00:57:38] 174 | |         self.into_inner().into_fd().into_raw()
[00:57:38] 175 | |     }
[00:57:38] 176 | | }
[00:57:38]     | |_- first implementation here
[00:57:38] 
[00:57:38] error[E0119]: conflicting implementations of trait `sys::unix_ext::io::IntoRawFd` for type `process::ChildStderr`:
[00:57:38]    --> /checkout/src/libstd/sys/unix/ext/process.rs:189:1
[00:57:38]     |
[00:57:38] 189 | / impl IntoRawFd for process::ChildStderr {
[00:57:38] 190 | |     fn into_raw_fd(self) -> RawFd {
[00:57:38] 191 | |         self.into_inner().into_fd().into_raw()
[00:57:38] 192 | |     }
[00:57:38] 193 | | }
[00:57:38]     | |_^ conflicting implementation for `process::ChildStderr`
[00:57:38]     | 
[00:57:38]    ::: /checkout/src/libstd/sys/redox/ext/process.rs
[00:57:38]     |
[00:57:38] 179 | / impl IntoRawFd for process::ChildStderr {
[00:57:38] 180 | |     fn into_raw_fd(self) -> RawFd {
[00:57:38] 181 | |         self.into_inner().into_fd().into_raw()
[00:57:38] 182 | |     }
[00:57:38] 183 | | }
[00:57:38]     | |_- first implementation here
[00:57:38] 
[00:57:38] error: Compilation failed, aborting rustdoc
[00:57:38] 
[00:57:38] [m[m[31m[1merror:[m Could not document `std`.
