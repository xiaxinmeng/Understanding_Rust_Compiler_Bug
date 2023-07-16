
error: couldn't read "\\\\?\\C:\\Users\\<snip>\\Source\\irondragon\\src\\gpioaccess\\proxyimpl/linux.rs": The filename, directory name, or volume label syntax is incorrect. (os error 123)
  --> \\?\C:\Users\<snip>\Source\irondragon\src\gpioaccess\mod.rs:68:9
   |
67 | #[path = "proxyimpl/nonlinux.rs"]
   | --------------------------------- verify the path separator being used matches the target platform (`/` vs `\`)
68 | pub mod proxyimpl;
   |         ^^^^^^^^^
