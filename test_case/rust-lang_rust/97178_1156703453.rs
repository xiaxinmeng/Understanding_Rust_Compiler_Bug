plain
configure: rust.debug-assertions := True
configure: rust.overflow-checks := True
configure: llvm.assertions      := True
configure: dist.missing-tools   := True
configure: build.configure-args := ['--enable-sccache', '--disable-manage-submodu ...
configure: writing `config.toml` in current directory
configure: 
configure: run `python /checkout/x.py --help`
configure: 
---
    Checking std_detect v0.1.5 (/checkout/library/stdarch/crates/std_detect)
    Checking miniz_oxide v0.4.0
    Checking object v0.26.2
    Checking addr2line v0.16.0
error: associated function has missing stability attribute
   --> library/std/src/os/windows/io/handle.rs:191:5
    |
191 | /     pub fn try_clone_to_owned(&self) -> crate::io::Result<OwnedHandle> {
192 | |         self.duplicate(0, false, c::DUPLICATE_SAME_ACCESS)
    | |_____^

error: associated function has missing stability attribute
error: associated function has missing stability attribute
   --> library/std/src/os/windows/io/socket.rs:110:5
    |
110 | /     pub fn try_clone_to_owned(&self) -> io::Result<OwnedSocket> {
111 | |         let mut info = unsafe { mem::zeroed::<c::WSAPROTOCOL_INFO>() };
112 | |         let result = unsafe {
113 | |             c::WSADuplicateSocketW(self.as_raw_socket(), c::GetCurrentProcessId(), &mut info)
156 | |         }
157 | |     }
    | |_____^

