plain
[RUSTC-TIMING] gimli test:false 3.433
[RUSTC-TIMING] object test:false 4.050
warning: dropping unsupported crate type `dylib` for target `wasm32-wasi`

error: type `Advice` from private dependency 'wasi' in public interface
  --> library/std/src/sys/wasi/fd.rs:99:5
   |
99 |     pub fn advise(&self, offset: u64, len: u64, advice: wasi::Advice) -> io::Result<()> {
   |
   |
   = note: `-D exported-private-dependencies` implied by `-D warnings`

error: type `Filestat` from private dependency 'wasi' in public interface
   --> library/std/src/sys/wasi/fd.rs:182:5
    |
182 |     pub fn filestat_get(&self) -> io::Result<wasi::Filestat> {


error: type `Filestat` from private dependency 'wasi' in public interface
   --> library/std/src/sys/wasi/fd.rs:202:5
202 | /     pub fn path_filestat_get(
203 | |         &self,
203 | |         &self,
204 | |         flags: wasi::Lookupflags,
205 | |         path: &str,
206 | |     ) -> io::Result<wasi::Filestat> {


error: type `Filestat` from private dependency 'wasi' in public interface
   --> library/std/src/sys/wasi/fs.rs:107:5
    |
107 |     pub fn as_wasi(&self) -> &wasi::Filestat {


error: type `Filetype` from private dependency 'wasi' in public interface
   --> library/std/src/sys/wasi/fs.rs:145:5
    |
145 |     pub fn bits(&self) -> wasi::Filetype {

[RUSTC-TIMING] std test:false 2.121
warning: `std` (lib) generated 1 warning
error: could not compile `std` (lib) due to 5 previous errors; 1 warning emitted
