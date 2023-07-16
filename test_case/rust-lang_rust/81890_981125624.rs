
021-11-28T17:13:11.5135303Z The bin target `test` in package `test v0.0.0 (D:\a\rust\rust\build\x86_64-pc-windows-msvc\stage2-tools\x86_64-pc-windows-msvc\tmp\cit\t1300\foo)` has the same output filename as the lib target `test` in package `test v0.0.0 (D:\a\rust\rust\build\x86_64-pc-windows-msvc\stage2-tools\x86_64-pc-windows-msvc\tmp\cit\t1300\foo)`.
2021-11-28T17:13:11.5137366Z Colliding filename is: D:\a\rust\rust\build\x86_64-pc-windows-msvc\stage2-tools\x86_64-pc-windows-msvc\tmp\cit\t1300\foo\target\release\deps\test.pdb
2021-11-28T17:13:11.5138476Z The targets should have unique names.
2021-11-28T17:13:11.5139250Z Consider changing their names to be unique or compiling them separately.
2021-11-28T17:13:11.5140516Z This may become a hard error in the future; see <https://github.com/rust-lang/cargo/issues/6313>.
2021-11-28T17:13:11.5141536Z warning: output filename collision.
2021-11-28T17:13:11.5143164Z The bin target `test` in package `test v0.0.0 (D:\a\rust\rust\build\x86_64-pc-windows-msvc\stage2-tools\x86_64-pc-windows-msvc\tmp\cit\t1300\foo)` has the same output filename as the lib target `test` in package `test v0.0.0 (D:\a\rust\rust\build\x86_64-pc-windows-msvc\stage2-tools\x86_64-pc-windows-msvc\tmp\cit\t1300\foo)`.
2021-11-28T17:13:11.5145380Z Colliding filename is: D:\a\rust\rust\build\x86_64-pc-windows-msvc\stage2-tools\x86_64-pc-windows-msvc\tmp\cit\t1300\foo\target\release\test.pdb
2021-11-28T17:13:11.5146482Z The targets should have unique names.
2021-11-28T17:13:11.5147438Z Consider changing their names to be unique or compiling them separately.
2021-11-28T17:13:11.5148495Z This may become a hard error in the future; see <https://github.com/rust-lang/cargo/issues/6313>.
[...]
021-11-28T17:13:11.5246422Z   = note: LINK : fatal error LNK1201: error writing to program database 'D:\a\rust\rust\build\x86_64-pc-windows-msvc\stage2-tools\x86_64-pc-windows-msvc\tmp\cit\t1300\foo\target\release\deps\test.pdb'; check for insufficient disk space, invalid path, or insufficient privilege
2021-11-28T17:13:11.5247882Z           
2021-11-28T17:13:11.5248143Z 
2021-11-28T17:13:11.5248698Z error: could not compile `test` due to previous error
